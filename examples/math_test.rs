// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use projectrigor::{Transform, glam::{Vec3, Quat}};
use std::f32::consts::PI;

fn main() {
    println!("=== ProjectRigor Math Module Test ===\n");

    // Test 1: Identity transform
    let identity = Transform::identity();
    println!("1. Identity Transform:");
    println!("   Position: {:?}", identity.position);
    println!("   Rotation: {:?}", identity.rotation);
    println!("   Scale: {:?}", identity.scale);
    println!();

    // Test 2: Transform a point
    let transform = Transform::from_position(Vec3::new(5.0, 10.0, 15.0));
    let point = Vec3::new(1.0, 2.0, 3.0);
    let transformed = transform.transform_point(point);
    println!("2. Transform Point:");
    println!("   Original: {:?}", point);
    println!("   Transform position: {:?}", transform.position);
    println!("   Transformed: {:?}", transformed);
    println!();

    // Test 3: Rotation
    let rotation = Quat::from_rotation_y(PI / 2.0); // 90 degrees around Y-axis
    let rotated_transform = Transform::from_position_rotation(Vec3::ZERO, rotation);
    let point_to_rotate = Vec3::new(1.0, 0.0, 0.0);
    let rotated = rotated_transform.transform_point(point_to_rotate);
    println!("3. Rotate Point 90° around Y-axis:");
    println!("   Original: {:?}", point_to_rotate);
    println!("   Rotated: {:?}", rotated);
    println!("   (X should become -Z)");
    println!();

    // Test 4: Combined transform
    let mut complex_transform = Transform::from_position(Vec3::new(10.0, 5.0, 0.0));
    complex_transform.rotate(Quat::from_rotation_z(PI / 4.0)); // 45 degrees
    complex_transform.translate(Vec3::new(5.0, 0.0, 0.0));
    
    println!("4. Combined Transform:");
    println!("   Final position: {:?}", complex_transform.position);
    println!("   Rotation applied: 45° around Z");
    println!();

    // Test 5: Transform to matrix
    let matrix = transform.to_matrix();
    println!("5. Transform to Matrix:");
    println!("   Matrix:\n{}", matrix);
    println!();

    println!("✓ All math tests completed successfully!");
}
