use anyhow::Result;

fn main() -> Result<()> {
    let result = divide_and_multiply(10, 5, 2)?;
    println!("Result: {}", result);
    Ok(())
}

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        anyhow::bail!("Division by zero");
    }
    Ok(a/b)
}

fn multiply(a: i32, b: i32) -> Result<i32> {
    if a == 0 || b == 0 {
        anyhow::bail!("Multiplication by zero");
    }
    Ok(a * b)
}

fn divide_and_multiply(a: i32, b: i32, c: i32) -> Result<i32> {
    let x = divide(a, b)?;
    let y = multiply(x, c)?;
    Ok(y)
}

