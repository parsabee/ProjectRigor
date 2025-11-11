// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#![allow(unexpected_cfgs)]

use clap::Parser;
use projectrigor::{App, RenderingMode};
use winit::event_loop::{ControlFlow, EventLoop};

/// ProjectRigor: A physics-based robotics and animation engine for Apple Silicon
#[derive(Parser, Debug)]
#[command(name = "ProjectRigor")]
#[command(version, about, long_about = None)]
struct Args {
    /// Rendering mode selection
    #[command(flatten)]
    render_mode: RenderMode,
}

#[derive(Debug, clap::Args)]
#[group(required = false, multiple = false)]
struct RenderMode {
    /// Use hardware-accelerated ray tracing (Metal ray tracing)
    #[arg(long, group = "mode")]
    hw_rt: bool,

    /// Use software ray tracing (CPU-based)
    #[arg(long, group = "mode")]
    sw_rt: bool,

    /// Use rasterization (default Metal rendering pipeline)
    #[arg(long, group = "mode")]
    raster: bool,
}

impl RenderMode {
    fn get_mode(&self) -> RenderingMode {
        if self.hw_rt {
            RenderingMode::HardwareRayTracing
        } else if self.sw_rt {
            RenderingMode::SoftwareRayTracing
        } else {
            // Default to rasterization
            RenderingMode::Rasterization
        }
    }
}

fn main() {
    let args = Args::parse();
    let render_mode = args.render_mode.get_mode();
    
    println!("Starting ProjectRigor with rendering mode: {:?}", render_mode);
    
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut app = App::new_with_render_mode(render_mode);
    event_loop.run_app(&mut app).unwrap();
}

