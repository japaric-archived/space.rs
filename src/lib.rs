//! Iterator-based `linspace` and `logspace` functions

#![deny(missing_docs, warnings)]
#![feature(plugin)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[plugin]
extern crate quickcheck_macros;

use std::num::{Float, self};

#[cfg(test)]
mod test;

/// Iterator that yields equally spaced numbers in the linear scale
#[derive(Copy)]
pub struct Linspace<T: Float> {
    start: T,
    state: uint,
    step: T,
    stop: uint,
}

impl<T> DoubleEndedIterator for Linspace<T> where T: Float {
    fn next_back(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            self.stop -= 1;
            Some(self.start + self.step * num::cast(self.stop).unwrap())
        }
    }
}

impl<T> Iterator for Linspace<T> where T: Float {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            let next = self.start + self.step * num::cast(self.state).unwrap();
            self.state += 1;
            Some(next)
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        let exact = self.stop - self.state;
        (exact, Some(exact))
    }
}

/// Iterator that yields equally spaced numbers in the logarithmic scale
#[derive(Copy)]
pub struct Logspace<T: Float> {
    start: T,
    state: uint,
    step: T,
    stop: uint,
}

impl<T> DoubleEndedIterator for Logspace<T> where T: Float {
    fn next_back(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            self.stop -= 1;
            Some((self.start + self.step * num::cast(self.stop).unwrap()).exp())
        }
    }
}

impl<T> Iterator for Logspace<T> where T: Float {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            let next = self.start + self.step * num::cast(self.state).unwrap();
            self.state += 1;
            Some(next.exp())
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        let exact = self.stop - self.state;
        (exact, Some(exact))
    }
}

/// Returns an iterator that yields `n` evenly spaced numbers over the `[start, end]` interval
///
/// # Panics
///
/// Panics if `end` < `start`
///
/// # Examples
///
/// **Note** These assertions will likely fail because of rounding errors. (In real applications
/// you shouldn't directly use equality between floats, but instead check that the absolute
/// difference is within some tolerance)
///
/// ``` ignore
/// assert_eq!(vec![2., 2.25, 2.5, 2.75, 3.], linspace(2., 3., 5).collect())
/// assert_eq!(vec![3., 2.75, 2.5, 2.25, 2.], linspace(2., 3., 5).rev().collect())
/// ```
pub fn linspace<T>(start: T, end: T, n: uint) -> Linspace<T> where T: Float {
    assert!(start <= end);

    let step = if n < 2 {
        // NB The value of `step` doesn't matter in these cases
        Float::zero()
    } else {
        (end - start) / num::cast(n - 1).unwrap()
    };

    Linspace {
        start: start,
        state: 0,
        step: step,
        stop: n,
    }
}

/// Logarithmic version of `linspace`
///
/// # Panics
///
/// Panics if `start` or `end` is a non-positive number, or if `end` < `start`
///
/// # Examples
///
/// **Note** These assertions will likely fail because of rounding errors. (In real applications
/// you shouldn't directly use equality between floats, but instead check that the absolute
/// difference is within some tolerance)
///
/// ``` ignore
/// assert_eq!(vec![0.1, 1., 10., 100.], logspace(0.1, 100., 4).collect())
/// assert_eq!(vec![100., 10., 1., 0.1], logspace(0.1, 100., 4).rev().collect())
/// ```
pub fn logspace<T>(start: T, end: T, n: uint) -> Logspace<T> where T: Float {
    assert!(start > Float::zero() && end > Float::zero() && start <= end);

    let (start, end) = (start.ln(), end.ln());

    let step = if n < 2 {
        // NB The value of `step` doesn't matter in these cases
        Float::zero()
    } else {
        (end - start) / num::cast(n - 1).unwrap()
    };

    Logspace {
        start: start,
        state: 0,
        step: step,
        stop: n,
    }
}
