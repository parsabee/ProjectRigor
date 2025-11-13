// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Performance monitoring and profiling utilities.
//!
//! This module is only active when compiled with the `perf` feature flag.
//! In release builds without the feature, all methods are no-ops with zero overhead.

#[cfg(feature = "perf")]
use std::collections::VecDeque;
#[cfg(feature = "perf")]
use std::time::{Duration, Instant};

#[cfg(feature = "perf")]
const WINDOW_SIZE: usize = 60; // 60 frame rolling average

/// Performance metrics for a single frame
#[cfg(feature = "perf")]
#[derive(Debug, Clone, Copy, Default)]
struct FrameMetrics {
    total_time: Duration,
    physics_time: Duration,
    query_time: Duration,
    render_time: Duration,
    buffer_allocs: u32,
    bytes_uploaded: u64,
}

/// Tracks performance over time with rolling averages
#[cfg(feature = "perf")]
pub struct PerfTracker {
    // Frame history
    frames: VecDeque<FrameMetrics>,
    
    // Current frame being tracked
    current: FrameMetrics,
    frame_start: Option<Instant>,
    subsystem_start: Option<Instant>,
    
    // External state (passed in)
    triangle_count: u32,
    
    // Cumulative totals since tracker creation
    total_allocations: u64,
    total_bytes_uploaded: u64,
    total_frees: u64,
    total_bytes_freed: u64,
    
    // Live memory tracking (current allocated - freed)
    current_memory_bytes: u64,
}

#[cfg(feature = "perf")]
impl PerfTracker {
    pub fn new() -> Self {
        Self {
            frames: VecDeque::with_capacity(WINDOW_SIZE),
            current: FrameMetrics::default(),
            frame_start: None,
            subsystem_start: None,
            triangle_count: 0,
            total_allocations: 0,
            total_bytes_uploaded: 0,
            total_frees: 0,
            total_bytes_freed: 0,
            current_memory_bytes: 0,
        }
    }
    
    /// Call at the start of each frame
    pub fn begin_frame(&mut self) {
        self.frame_start = Some(Instant::now());
        self.current = FrameMetrics::default();
    }
    
    /// Mark start of a subsystem (physics, query, render)
    pub fn mark_start(&mut self) {
        self.subsystem_start = Some(Instant::now());
    }
    
    /// Record physics time
    pub fn mark_physics(&mut self) {
        if let Some(start) = self.subsystem_start.take() {
            self.current.physics_time = start.elapsed();
        }
    }
    
    /// Record query time
    pub fn mark_query(&mut self) {
        if let Some(start) = self.subsystem_start.take() {
            self.current.query_time = start.elapsed();
        }
    }
    
    /// Record render time
    pub fn mark_render(&mut self) {
        if let Some(start) = self.subsystem_start.take() {
            self.current.render_time = start.elapsed();
        }
    }
    
    /// Record a buffer allocation
    pub fn record_allocation(&mut self, bytes: u64) {
        self.current.buffer_allocs += 1;
        self.current.bytes_uploaded += bytes;
        self.total_allocations += 1;
        self.total_bytes_uploaded += bytes;
        self.current_memory_bytes += bytes;
    }
    
    /// Record a buffer free
    pub fn record_free(&mut self, bytes: u64) {
        self.total_frees += 1;
        self.total_bytes_freed += bytes;
        self.current_memory_bytes = self.current_memory_bytes.saturating_sub(bytes);
    }
    
    /// Set triangle count for current frame
    pub fn set_triangle_count(&mut self, count: u32) {
        self.triangle_count = count;
    }
    
    /// Call at end of frame
    pub fn end_frame(&mut self) {
        if let Some(start) = self.frame_start.take() {
            self.current.total_time = start.elapsed();
        }
        
        // Add to history
        self.frames.push_back(self.current);
        if self.frames.len() > WINDOW_SIZE {
            self.frames.pop_front();
        }
    }
    
    /// Calculate current FPS (based on last frame)
    pub fn current_fps(&self) -> f64 {
        if let Some(last) = self.frames.back() {
            if last.total_time.as_secs_f64() > 0.0 {
                return 1.0 / last.total_time.as_secs_f64();
            }
        }
        0.0
    }
    
    /// Calculate average FPS over window
    pub fn avg_fps(&self) -> f64 {
        if self.frames.is_empty() {
            return 0.0;
        }
        let avg_time = self.avg_frame_time();
        if avg_time.as_secs_f64() > 0.0 {
            1.0 / avg_time.as_secs_f64()
        } else {
            0.0
        }
    }
    
    /// Average frame time
    fn avg_frame_time(&self) -> Duration {
        if self.frames.is_empty() {
            return Duration::ZERO;
        }
        let total: Duration = self.frames.iter().map(|f| f.total_time).sum();
        total / self.frames.len() as u32
    }
    
    /// Min/Max frame time (for detecting stutter)
    fn min_max_frame_time(&self) -> (Duration, Duration) {
        if self.frames.is_empty() {
            return (Duration::ZERO, Duration::ZERO);
        }
        let min = self.frames.iter().map(|f| f.total_time).min().unwrap();
        let max = self.frames.iter().map(|f| f.total_time).max().unwrap();
        (min, max)
    }
    
    /// Average subsystem times
    fn avg_subsystems(&self) -> (Duration, Duration, Duration) {
        if self.frames.is_empty() {
            return (Duration::ZERO, Duration::ZERO, Duration::ZERO);
        }
        let physics: Duration = self.frames.iter().map(|f| f.physics_time).sum();
        let query: Duration = self.frames.iter().map(|f| f.query_time).sum();
        let render: Duration = self.frames.iter().map(|f| f.render_time).sum();
        let n = self.frames.len() as u32;
        (physics / n, query / n, render / n)
    }
    
    /// Average allocations
    fn avg_allocations(&self) -> (f64, f64) {
        if self.frames.is_empty() {
            return (0.0, 0.0);
        }
        let total_allocs: u32 = self.frames.iter().map(|f| f.buffer_allocs).sum();
        let total_bytes: u64 = self.frames.iter().map(|f| f.bytes_uploaded).sum();
        let n = self.frames.len() as f64;
        (total_allocs as f64 / n, total_bytes as f64 / n)
    }
    
    /// Print summary to terminal
    pub fn print_summary(&self) {
        let (min_ft, max_ft) = self.min_max_frame_time();
        let (phys, query, render) = self.avg_subsystems();
        let (allocs, bytes) = self.avg_allocations();
        
        println!("\n╔═══════════════════════════════════════════════════════╗");
        println!("║           PERFORMANCE METRICS                         ║");
        println!("╚═══════════════════════════════════════════════════════╝");
        
        println!("  FPS:          {:<6.1} (current)", self.current_fps());
        println!("                {:<6.1} (avg over {} frames)", self.avg_fps(), self.frames.len());
        
        println!(" ───────────────────────────────────────────────────────");
        println!("  Frame Time:   {:<6.2} ms (avg)", self.avg_frame_time().as_secs_f64() * 1000.0);
        println!("                {:<6.2} ms (min)", min_ft.as_secs_f64() * 1000.0);
        println!("                {:<6.2} ms (max)", max_ft.as_secs_f64() * 1000.0);
        
        println!(" ───────────────────────────────────────────────────────");
        println!("  Breakdown:");
        println!("    Physics:    {:<6.2} ms", phys.as_secs_f64() * 1000.0);
        println!("    Query:      {:<6.2} ms", query.as_secs_f64() * 1000.0);
        println!("    Render:     {:<6.2} ms", render.as_secs_f64() * 1000.0);
        
        println!(" ───────────────────────────────────────────────────────");
        println!("  GPU (per frame):");
        println!("    Allocations: {:<6.1} per frame", allocs);
        println!("    Upload:      {:<6.1} KB per frame", bytes / 1024.0);
        println!("    Triangles:   {:<6}", self.triangle_count);
        println!("    Current Mem: {:<6.1} MB", self.current_memory_bytes as f64 / (1024.0 * 1024.0));
        
        println!(" ───────────────────────────────────────────────────────");
        println!("  GPU (cumulative):");
        println!("    Total Allocs: {:<6}", self.total_allocations);
        println!("    Total Upload: {:<6.1} MB", self.total_bytes_uploaded as f64 / (1024.0 * 1024.0));
        println!("    Total Frees:  {:<6}", self.total_frees);
        println!("    Total Freed:  {:<6.1} MB", self.total_bytes_freed as f64 / (1024.0 * 1024.0));
        println!("    Net Memory:   {:<6.1} MB", (self.total_bytes_uploaded as i64 - self.total_bytes_freed as i64) as f64 / (1024.0 * 1024.0));
        println!("    Current Alloc: {:<6.1} MB\n", self.current_memory_bytes as f64 / (1024.0 * 1024.0));
    }
}

#[cfg(feature = "perf")]
impl Default for PerfTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "perf")]
impl Drop for PerfTracker {
    fn drop(&mut self) {
        println!("\n=== Performance Summary (on drop) ===");
        self.print_summary();
    }
}

// No-op stub when perf feature is disabled
#[cfg(not(feature = "perf"))]
pub struct PerfTracker;

#[cfg(not(feature = "perf"))]
impl PerfTracker {
    #[inline(always)]
    pub fn new() -> Self { Self }
    
    #[inline(always)]
    pub fn begin_frame(&mut self) {}
    
    #[inline(always)]
    pub fn mark_start(&mut self) {}
    
    #[inline(always)]
    pub fn mark_physics(&mut self) {}
    
    #[inline(always)]
    pub fn mark_query(&mut self) {}
    
    #[inline(always)]
    pub fn mark_render(&mut self) {}
    
    #[inline(always)]
    pub fn record_allocation(&mut self, _bytes: u64) {}
    
    #[inline(always)]
    pub fn record_free(&mut self, _bytes: u64) {}
    
    #[inline(always)]
    pub fn set_triangle_count(&mut self, _count: u32) {}
    
    #[inline(always)]
    pub fn end_frame(&mut self) {}
    
    #[inline(always)]
    pub fn print_summary(&self) {}
}

#[cfg(not(feature = "perf"))]
impl Default for PerfTracker {
    fn default() -> Self {
        Self
    }
}

#[cfg(all(test, feature = "perf"))]
mod tests {
    use super::*;
    use std::thread;
    
    /// Helper to create a controlled frame with known timings
    fn simulate_frame(tracker: &mut PerfTracker, 
                      physics_ms: u64, 
                      query_ms: u64, 
                      render_ms: u64,
                      allocations: &[(u64, u64)]) { // (count, bytes)
        tracker.begin_frame();
        
        // Physics
        tracker.mark_start();
        thread::sleep(Duration::from_millis(physics_ms));
        tracker.mark_physics();
        
        // Query
        tracker.mark_start();
        thread::sleep(Duration::from_millis(query_ms));
        tracker.mark_query();
        
        // Render
        tracker.mark_start();
        thread::sleep(Duration::from_millis(render_ms));
        tracker.mark_render();
        
        // Allocations
        for &(count, bytes) in allocations {
            for _ in 0..count {
                tracker.record_allocation(bytes);
            }
        }
        
        tracker.end_frame();
    }
    
    #[test]
    fn test_single_frame() {
        let mut tracker = PerfTracker::new();
        tracker.begin_frame();
        thread::sleep(Duration::from_millis(10));
        tracker.end_frame();
        
        assert!(tracker.current_fps() > 0.0);
        assert!(tracker.avg_frame_time() >= Duration::from_millis(10));
    }
    
    #[test]
    fn test_rolling_average() {
        let mut tracker = PerfTracker::new();
        
        // Record 100 frames
        for _ in 0..100 {
            tracker.begin_frame();
            thread::sleep(Duration::from_millis(1));
            tracker.end_frame();
        }
        
        // Should only keep last 60
        assert_eq!(tracker.frames.len(), 60);
    }
    
    #[test]
    fn test_subsystem_timing() {
        let mut tracker = PerfTracker::new();
        tracker.begin_frame();
        
        tracker.mark_start();
        thread::sleep(Duration::from_millis(5));
        tracker.mark_physics();
        
        tracker.mark_start();
        thread::sleep(Duration::from_millis(3));
        tracker.mark_query();
        
        tracker.end_frame();
        
        let (phys, query, _) = tracker.avg_subsystems();
        assert!(phys >= Duration::from_millis(5));
        assert!(query >= Duration::from_millis(3));
    }
    
    #[test]
    fn test_allocation_tracking() {
        let mut tracker = PerfTracker::new();
        tracker.begin_frame();
        tracker.record_allocation(1024);
        tracker.record_allocation(2048);
        tracker.end_frame();
        
        let (allocs, bytes) = tracker.avg_allocations();
        assert_eq!(allocs, 2.0);
        assert_eq!(bytes, 3072.0);
    }
    
    #[test]
    fn test_multiple_frames_allocation_tracking() {
        let mut tracker = PerfTracker::new();
        
        // Frame 1: 3 allocations (simulating triangle_buffer, params_buffer, count_buffer)
        tracker.begin_frame();
        tracker.record_allocation(48 * 1024); // triangle buffer 48KB
        tracker.record_allocation(128);        // params buffer 128 bytes
        tracker.record_allocation(4);          // count buffer 4 bytes
        tracker.end_frame();
        
        let (allocs, bytes) = tracker.avg_allocations();
        assert_eq!(allocs, 3.0, "First frame should have 3 allocations");
        assert_eq!(bytes, (48 * 1024 + 128 + 4) as f64);
        
        // Frame 2: 0 allocations (buffer reuse)
        tracker.begin_frame();
        tracker.end_frame();
        
        let (allocs, bytes) = tracker.avg_allocations();
        assert_eq!(allocs, 1.5, "Average should be (3 + 0) / 2 = 1.5");
        assert_eq!(bytes, ((48 * 1024 + 132) / 2) as f64);
        
        // Frame 3: 0 allocations (buffer reuse)
        tracker.begin_frame();
        tracker.end_frame();
        
        let (allocs, bytes) = tracker.avg_allocations();
        assert_eq!(allocs, 1.0, "Average should be (3 + 0 + 0) / 3 = 1.0");
        assert_eq!(bytes, ((48 * 1024 + 132) / 3) as f64);
    }
    
    #[test]
    fn test_allocation_tracking_shows_zero_after_warmup() {
        let mut tracker = PerfTracker::new();
        
        // Frame 1: Initial allocations
        tracker.begin_frame();
        tracker.record_allocation(1024);
        tracker.record_allocation(2048);
        tracker.record_allocation(512);
        tracker.end_frame();
        
        // Frames 2-60: No allocations (buffer reuse)
        for _ in 0..59 {
            tracker.begin_frame();
            tracker.end_frame();
        }
        
        let (allocs, bytes) = tracker.avg_allocations();
        // After 60 frames with only first frame having allocations:
        // allocs = 3/60 = 0.05
        assert!(allocs < 0.1, "After buffer reuse warmup, allocations should be near zero, got {}", allocs);
        assert!(bytes < 100.0, "After buffer reuse warmup, bytes should be near zero, got {}", bytes);
    }
    
    #[test]
    fn test_continuous_allocations_show_correctly() {
        let mut tracker = PerfTracker::new();
        
        // Simulate what happens WITHOUT buffer reuse - every frame has 3 allocations
        for _ in 0..60 {
            tracker.begin_frame();
            tracker.record_allocation(48 * 1024); // triangle buffer
            tracker.record_allocation(128);        // params buffer
            tracker.record_allocation(4);          // count buffer
            tracker.end_frame();
        }
        
        let (allocs, bytes) = tracker.avg_allocations();
        assert_eq!(allocs, 3.0, "Should show 3 allocations per frame");
        assert_eq!(bytes, (48 * 1024 + 132) as f64, "Should show correct byte count");
    }
    
    #[test]
    fn test_fps_accuracy() {
        let mut tracker = PerfTracker::new();
        
        // Simulate 10 frames at ~16.67ms each (60 FPS)
        for _ in 0..10 {
            simulate_frame(&mut tracker, 1, 1, 14, &[]);
        }
        
        let fps = tracker.avg_fps();
        // Allow 15% tolerance due to sleep imprecision on different systems
        assert!(fps >= 51.0 && fps <= 69.0, "Expected ~60 FPS, got {}", fps);
    }
    
    #[test]
    fn test_frame_time_min_max() {
        let mut tracker = PerfTracker::new();
        
        // Create frames with varying times
        simulate_frame(&mut tracker, 1, 0, 4, &[]); // ~5ms
        simulate_frame(&mut tracker, 2, 0, 8, &[]); // ~10ms
        simulate_frame(&mut tracker, 1, 0, 2, &[]); // ~3ms
        
        let (min, max) = tracker.min_max_frame_time();
        
        // Min should be around 3ms, max around 10ms (with tolerance for sleep imprecision)
        assert!(min >= Duration::from_millis(3) && min <= Duration::from_millis(6),
                "Min expected 3-6ms, got {}ms", min.as_millis());
        assert!(max >= Duration::from_millis(9) && max <= Duration::from_millis(13),
                "Max expected 9-13ms, got {}ms", max.as_millis());
    }
    
    #[test]
    fn test_subsystem_breakdown_accuracy() {
        let mut tracker = PerfTracker::new();
        
        // Simulate 5 frames with known subsystem times
        for _ in 0..5 {
            simulate_frame(&mut tracker, 2, 3, 5, &[]);
        }
        
        let (physics, query, render) = tracker.avg_subsystems();
        
        // Each should be within 1ms of target (tolerance for sleep imprecision)
        assert!((physics.as_millis() as i64 - 2).abs() <= 1, 
                "Physics expected ~2ms, got {}ms", physics.as_millis());
        assert!((query.as_millis() as i64 - 3).abs() <= 1,
                "Query expected ~3ms, got {}ms", query.as_millis());
        assert!((render.as_millis() as i64 - 5).abs() <= 1,
                "Render expected ~5ms, got {}ms", render.as_millis());
    }
    
    #[test]
    fn test_allocation_averaging() {
        let mut tracker = PerfTracker::new();
        
        // Frame 1: 2 allocations of 1024 bytes each
        simulate_frame(&mut tracker, 0, 0, 1, &[(2, 1024)]);
        
        // Frame 2: 3 allocations of 512 bytes each
        simulate_frame(&mut tracker, 0, 0, 1, &[(3, 512)]);
        
        // Frame 3: 1 allocation of 4096 bytes
        simulate_frame(&mut tracker, 0, 0, 1, &[(1, 4096)]);
        
        let (avg_allocs, avg_bytes) = tracker.avg_allocations();
        
        // Average allocations: (2 + 3 + 1) / 3 = 2.0
        assert_eq!(avg_allocs, 2.0);
        
        // Average bytes: (2048 + 1536 + 4096) / 3 = 2560.0
        assert_eq!(avg_bytes, 2560.0);
    }
    
    #[test]
    fn test_triangle_count_tracking() {
        let mut tracker = PerfTracker::new();
        
        tracker.begin_frame();
        tracker.set_triangle_count(1234);
        tracker.end_frame();
        
        // Triangle count should be retained
        assert_eq!(tracker.triangle_count, 1234);
        
        tracker.begin_frame();
        tracker.set_triangle_count(5678);
        tracker.end_frame();
        
        // Should update to new value
        assert_eq!(tracker.triangle_count, 5678);
    }
    
    #[test]
    fn test_zero_frames_safety() {
        let tracker = PerfTracker::new();
        
        // Should not panic with no frames
        assert_eq!(tracker.current_fps(), 0.0);
        assert_eq!(tracker.avg_fps(), 0.0);
        assert_eq!(tracker.avg_frame_time(), Duration::ZERO);
        
        let (min, max) = tracker.min_max_frame_time();
        assert_eq!(min, Duration::ZERO);
        assert_eq!(max, Duration::ZERO);
        
        let (phys, query, render) = tracker.avg_subsystems();
        assert_eq!(phys, Duration::ZERO);
        assert_eq!(query, Duration::ZERO);
        assert_eq!(render, Duration::ZERO);
        
        let (allocs, bytes) = tracker.avg_allocations();
        assert_eq!(allocs, 0.0);
        assert_eq!(bytes, 0.0);
    }
    
    #[test]
    fn test_window_overflow() {
        let mut tracker = PerfTracker::new();
        
        // Add more frames than window size
        for i in 0..100 {
            tracker.begin_frame();
            tracker.set_triangle_count(i as u32);
            thread::sleep(Duration::from_millis(1));
            tracker.end_frame();
        }
        
        // Should only keep WINDOW_SIZE frames
        assert_eq!(tracker.frames.len(), WINDOW_SIZE);
        
        // Latest triangle count should be preserved (not part of rolling window)
        assert_eq!(tracker.triangle_count, 99);
    }
    
    #[test]
    fn test_realistic_workload_simulation() {
        let mut tracker = PerfTracker::new();
        
        // Simulate 60 frames of realistic game loop
        // Physics: 0.5ms, Query: 0.1ms, Render: 7.4ms = 8ms total (125 FPS)
        // 3 GPU allocations per frame, 64KB total
        for _ in 0..60 {
            tracker.begin_frame();
            
            tracker.mark_start();
            thread::sleep(Duration::from_micros(500));
            tracker.mark_physics();
            
            tracker.mark_start();
            thread::sleep(Duration::from_micros(100));
            tracker.mark_query();
            
            tracker.mark_start();
            thread::sleep(Duration::from_millis(7));
            thread::sleep(Duration::from_micros(400));
            tracker.mark_render();
            
            // 3 allocations: triangle buffer, params buffer, count buffer
            tracker.record_allocation(48 * 1024); // 48KB
            tracker.record_allocation(8 * 1024);  // 8KB
            tracker.record_allocation(8 * 1024);  // 8KB
            
            tracker.set_triangle_count(1000);
            
            tracker.end_frame();
        }
        
        let fps = tracker.avg_fps();
        let (physics, query, render) = tracker.avg_subsystems();
        let (allocs, bytes) = tracker.avg_allocations();
        
        // FPS should be around 125 (8ms frame time), allow wide tolerance for CI
        assert!(fps >= 100.0 && fps <= 150.0, "Expected ~125 FPS, got {}", fps);
        
        // Subsystem times (with tolerance)
        assert!(physics.as_micros() >= 400 && physics.as_micros() <= 700);
        assert!(query.as_micros() >= 50 && query.as_micros() <= 250);
        assert!(render.as_millis() >= 7 && render.as_millis() <= 9);
        
        // Allocations should be exact
        assert_eq!(allocs, 3.0);
        assert_eq!(bytes, 64.0 * 1024.0);
        assert_eq!(tracker.triangle_count, 1000);
    }
}
