use anyhow::{anyhow, Context, Result};
use std::fs;


fn main() -> Result<()> {
    let content = read_file("my_file.txt")?;
    println!("File content: {}", content);

    let area = calculate_area(4, 5)?;
    println!("Area: {}", area);

    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| 
        format!("Failed to read file: {}", path))
}

fn calculate_area(width: u32, height: u32) -> Result<u32> {
    if width == 0 || height == 0 {
        return Err(anyhow!("Invalid dimensions: width or height is zero"));
    }
    Ok(width * height)
}