// tests/test_main.rs

use std::fs::File;
use std::io::Write;

use std::process::Command;

#[test]
fn test_file_reading() {
    // Create a temporary file with some content
    let mut temp_file = File::create("temp.txt").unwrap();
    writeln!(temp_file, "Line 1").unwrap();
    writeln!(temp_file, "Line 2").unwrap();
    writeln!(temp_file, "Line 3").unwrap();
    drop(temp_file);

    // Run the program with the temporary file as an argument
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("temp.txt")
        .output()
        .expect("Failed to execute command");

    // Assert that the program output matches the expected output
    let expected_output = "Line 1\nLine 2\nLine 3\n";
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

    // Clean up the temporary file
    std::fs::remove_file("temp.txt").unwrap();
}

#[test]
fn test_file_not_found() {
    // Run the program with a nonexistent file as an argument
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("nonexistent.txt")
        .output()
        .expect("Failed to execute command");

    // Check if the program exited with a non-zero status code
    assert!(!output.status.success());

    // Check if the stderr contains the expected error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("File not found"));
}