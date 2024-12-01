use std::ops::{Div, Mul, Rem};

pub fn gcd<T>(a: T, b: T) -> T
where
    T: PartialEq + Eq + Rem<Output = T> + From<u8> + Copy,
{
    if b == T::from(0u8) {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq + Eq + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + From<u8> + Copy,
{
    a / gcd(a, b) * b
}

pub fn lcm_of_vec<T>(numbers: &[T]) -> T
where
    T: PartialEq + Eq + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + From<u8> + Copy,
{
    numbers.iter().fold(T::from(1u8), |acc, &num| lcm(acc, num))
}
