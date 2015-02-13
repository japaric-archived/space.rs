//! Iterator-based `linspace` and `logspace` functions

#![cfg_attr(test, plugin(quickcheck_macros))]
#![deny(missing_docs, warnings)]
#![feature(plugin)]

#[cfg(test)]
extern crate quickcheck;
extern crate cast;

use cast::CastTo;
use std::num::Float;

#[cfg(test)]
mod test;

/// Iterator that yields equally spaced numbers in the linear scale
#[derive(Copy)]
pub struct Linspace<T> where T: cast::Float {
    start: T,
    state: usize,
    step: T,
    stop: usize,
}

impl<T> DoubleEndedIterator for Linspace<T> where T: cast::Float {
    fn next_back(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            self.stop -= 1;
            Some(self.start + self.step * self.stop.to::<T>())
        }
    }
}

impl<T> Iterator for Linspace<T> where T: cast::Float {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            let next = self.start + self.step * self.state.to::<T>();
            self.state += 1;
            Some(next)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.stop - self.state;
        (exact, Some(exact))
    }
}

/// Iterator that yields equally spaced numbers in the logarithmic scale
#[derive(Copy)]
pub struct Logspace<T> where T: cast::Float {
    start: T,
    state: usize,
    step: T,
    stop: usize,
}

impl<T> DoubleEndedIterator for Logspace<T> where T: cast::Float {
    fn next_back(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            self.stop -= 1;
            Some((self.start + self.step * self.stop.to::<T>()).exp())
        }
    }
}

impl<T> Iterator for Logspace<T> where T: cast::Float {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.state == self.stop {
            None
        } else {
            let next = self.start + self.step * self.state.to::<T>();
            self.state += 1;
            Some(next.exp())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
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
/// assert_eq!(vec![2., 2.25, 2.5, 2.75, 3.], linspace(2., 3., 5).collect::<Vec<_>>())
/// assert_eq!(vec![3., 2.75, 2.5, 2.25, 2.], linspace(2., 3., 5).rev().collect::<Vec<_>>())
/// ```
pub fn linspace<T>(start: T, end: T, n: usize) -> Linspace<T> where T: cast::Float {
    assert!(start <= end);

    let step = if n < 2 {
        // NB The value of `step` doesn't matter in these cases
        0.to::<T>()
    } else {
        (end - start) / (n - 1).to::<T>()
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
/// assert_eq!(vec![0.1, 1., 10., 100.], logspace(0.1, 100., 4).collect::<Vec<_>>())
/// assert_eq!(vec![100., 10., 1., 0.1], logspace(0.1, 100., 4).rev().collect::<Vec<_>>())
/// ```
pub fn logspace<T>(start: T, end: T, n: usize) -> Logspace<T> where T: cast::Float {
    let _0 = 0.to::<T>();

    assert!(start > _0 && end > _0 && start <= end);

    let (start, end) = (start.ln(), end.ln());

    let step = if n < 2 {
        // NB The value of `step` doesn't matter in these cases
        _0
    } else {
        (end - start) / (n - 1).to::<T>()
    };

    Logspace {
        start: start,
        state: 0,
        step: step,
        stop: n,
    }
}
