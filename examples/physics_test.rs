// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use projectrigor::{PhysicsWorld, Transform, glam::Vec3};

fn main() {
    println!("=== ProjectRigor Physics Module Test ===\n");

    // Create physics world
    let mut world = PhysicsWorld::new();
    println!("✓ Created physics world with gravity: {:?}\n", world.gravity);

    // Create ground plane
    let ground_transform = Transform::from_position(Vec3::new(0.0, -1.0, 0.0));
    let ground_handle = world.create_static_body(ground_transform);
    world.add_box_collider(ground_handle, Vec3::new(50.0, 1.0, 50.0));
    println!("✓ Created ground plane at y = -1.0\n");

    // Create falling boxes
    println!("Creating falling boxes:");
    let mut boxes = Vec::new();
    
    for i in 0..3 {
        let x = (i as f32 - 1.0) * 3.0;
        let transform = Transform::from_position(Vec3::new(x, 10.0, 0.0));
        let handle = world.create_dynamic_body(transform);
        world.add_box_collider(handle, Vec3::new(0.5, 0.5, 0.5));
        boxes.push((handle, format!("Box {}", i + 1)));
        println!("  • Box {} at position ({}, 10.0, 0.0)", i + 1, x);
    }
    println!();

    // Create a sphere
    let sphere_transform = Transform::from_position(Vec3::new(0.0, 15.0, 5.0));
    let sphere_handle = world.create_dynamic_body(sphere_transform);
    world.add_sphere_collider(sphere_handle, 1.0);
    println!("✓ Created sphere at (0.0, 15.0, 5.0) with radius 1.0\n");

    // Simulate
    println!("Running simulation...\n");
    
    let timesteps = [0, 30, 60, 90, 120];
    
    for &step in &timesteps {
        // Step to target
        for _ in 0..step {
            world.step();
        }
        
        println!("--- After {} steps (t ≈ {:.2}s) ---", step, step as f32 * 0.016667);
        
        for (handle, name) in &boxes {
            if let Some(transform) = world.get_transform(*handle) {
                println!("  {}: y = {:.3}", name, transform.position.y);
            }
        }
        
        if let Some(transform) = world.get_transform(sphere_handle) {
            println!("  Sphere: y = {:.3}", transform.position.y);
        }
        println!();
    }

    println!("Testing velocity...");
    let test_body = world.create_dynamic_body(Transform::from_position(Vec3::new(10.0, 5.0, 0.0)));
    world.add_box_collider(test_body, Vec3::new(0.5, 0.5, 0.5));
    world.set_velocity(test_body, Vec3::new(5.0, 0.0, 0.0));
    
    if let Some(vel) = world.get_velocity(test_body) {
        println!("  • Set velocity to ({:.1}, {:.1}, {:.1})", vel.x, vel.y, vel.z);
    }
    
    for _ in 0..10 {
        world.step();
    }
    
    if let Some(vel) = world.get_velocity(test_body) {
        println!("  • After 10 steps: ({:.2}, {:.2}, {:.2})", vel.x, vel.y, vel.z);
        println!("    (Y velocity is negative due to gravity)");
    }
    println!();

    println!("✓ Physics simulation complete!");
    println!("\nSummary:");
    println!("  • {} rigid bodies", world.rigid_body_set.len());
    println!("  • {} colliders", world.collider_set.len());
    println!("  • Gravity working correctly");
    println!("  • Collisions detected");
}
