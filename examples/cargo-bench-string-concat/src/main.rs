#![feature(test)]

extern crate test;

// 1. most trivial: iterate over nums
// convert nums to string and push
// them to final result string.
//
// input = 10
// output = 0123456789
fn create_series_concat_first(val: i64) -> String {
    let mut result = String::new();

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

// 2. Pre-decide the size of the final result string.
fn create_series_concat_second(val: i64) -> String {
    let length: usize = val.to_string().len(); // `0` = 1 | `12` = 1 | `333` = 3
    let capacity: usize = (val - 1) as usize * length;

    let mut result = String::with_capacity(capacity);

    for i in 0..val {
        result.push_str(&i.to_string());
    }
    result
}

// 3. Create a vector of numbers and using fold
// to generate the resultant string.
// `fold` is a consuming iterator adaptor which applies a function
// to each element of the iteration, accumulating the result into
// a new value.
fn create_series_concat_third(val: i64) -> String {
    let mut vec: Vec<i64> = Vec::new();

    for i in 0..val {
        vec.push(i)
    }

    vec.iter().fold(String::new(), |accumulator, s| {
        format!("{}{}", accumulator, s)
    })
}

// 4. Heaplessly converting numbers into their string
fn create_series_concat_fourth(val: i64) -> String {
    use numtoa::NumToA;

    let mut buffer = [0u8; 20];
    let length: usize = val.to_string().len();
    let capacity: usize = (val - 1) as usize * length;
    let mut result = String::with_capacity(capacity);

    for i in 0..val {
        result.push_str(i.numtoa_str(10, &mut buffer));
    }

    result
}

fn main() {
    let mut res = create_series_concat_first(11);
    println!("{}", res);

    res = create_series_concat_second(130);
    println!("{}", res);

    res = create_series_concat_third(33);
    println!("{}", res);

    res = create_series_concat_fourth(25);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn benchmark_first(b: &mut Bencher) {
        b.iter(|| create_series_concat_first(1000));
    }

    #[bench]
    fn benchmark_second(b: &mut Bencher) {
        b.iter(|| create_series_concat_second(1000));
    }

    #[bench]
    fn benchmark_third(b: &mut Bencher) {
        b.iter(|| create_series_concat_third(1000));
    }

    #[bench]
    fn benchmark_fourth(b: &mut Bencher) {
        b.iter(|| create_series_concat_fourth(1000));
    }
}
