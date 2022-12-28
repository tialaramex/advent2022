use jungle::readfile;

use core::ops::Add;
use core::ops::AddAssign;
use std::iter::Sum;
use std::str::FromStr;

#[derive(Copy, Clone, Default)]
struct Five(i64);

impl FromStr for Five {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;
        for digit in s.bytes() {
            value *= 5;
            value += match digit {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => {
                    return Err("Impossible digit in SNAFU");
                }
            };
        }
        Ok(Five(value))
    }
}

impl Add for Five {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Five {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl Sum for Five {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Default::default(), |a, b| a + b)
    }
}

use std::fmt;
impl fmt::Display for Five {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut value = self.0;
        while value != 0 {
            let digit = (value + 2) % 5 - 2;
            s.insert(
                0,
                match digit {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => {
                        panic!("Inconceivable!");
                    }
                },
            );
            value -= digit;
            value /= 5;
        }
        f.write_str(&s)?;
        Ok(())
    }
}

pub fn a() {
    let ctxt = readfile("25");
    let sum: Five = ctxt.lines().map(|l| l.parse().unwrap()).sum();
    println!("{sum}");
}

pub fn b() {
    println!("Happy Christmas!");
}
