use rayon::prelude::*;

/// Filtering elements in a collection can also be parallelized
/// with Rayon. In this example we want to extract the even numbers
/// from a vector.
pub fn extract_even_numbers_from_a_vector() {
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = numbers.par_iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even_numbers);
}

/// Compute the square of each element in a vector
pub fn compute_square_of_element() {
    let numbers = vec![2, 3, 5, 7, 9];
    let squares: Vec<_> = numbers.par_iter().map(|x| x * x).collect();
    println!("{:?}", squares);
}

/// Compute the sum of elements in a vector
pub fn sum_elements() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.par_iter().cloned().reduce(|| 0, |a, b| a + b);
    println!("Sum: {}", sum);
}
