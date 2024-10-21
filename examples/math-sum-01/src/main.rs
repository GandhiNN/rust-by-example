// takes reference to iterables and return the sum as owned i32 type
fn sum(numbers: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for &number in numbers.iter() {
        total += number;
    }
    total
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result: i32 = sum(&numbers);
    println!("Sum: {}", result);
}
