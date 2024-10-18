use std::fmt;

pub enum Term {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(i32),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Fizz => f.write_str("Fizz"),
            Term::Buzz => f.write_str("Buzz"),
            Term::FizzBuzz => f.write_str("FizzBuzz"),
            Term::Number(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub fn fizz_buzz_enhanced(i: i32) -> Term {
    if i % 15 == 0 {
        Term::FizzBuzz
    } else if i % 5 == 0 {
        Term::Buzz
    } else if i % 3 == 0 {
        Term::Fizz
    } else {
        Term::Number(i)
    }
}
