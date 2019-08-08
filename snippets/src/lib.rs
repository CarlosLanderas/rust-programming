pub mod closures;
pub mod collections;
pub mod cow;
pub mod generic_operators;
pub mod indexes;
pub mod iterators;
pub mod operators;

use std::ops::{Add, Mul};

pub fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total: N = Default::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0)
}
