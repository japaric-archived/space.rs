macro_rules! enforce {
    ($($e:expr),+,) => {
        if $(!$e)||+ {
            return TestResult::discard();
        }
    }
}

macro_rules! test {
    ($e:expr) => {
        TestResult::from_bool($e)
    }
}

trait IsClose {
    fn is_close(self, rhs: Self) -> bool;
}

macro_rules! float {
    ($($ty:ident),+) => {$(
        mod $ty {
            impl ::test::IsClose for $ty {
                fn is_close(self, rhs: $ty) -> bool {
                    const TOL: $ty = 1e-3;

                    (self - rhs).abs() < TOL
                }
            }

            mod linspace {
                use quickcheck::TestResult;

                mod rev {
                    use quickcheck::TestResult;

                    // Check that `linspace(..).rev()` yields evenly spaced numbers
                    #[quickcheck]
                    fn evenly_spaced(start: $ty, end: $ty, n: usize) -> TestResult {
                        use test::IsClose;

                        enforce! {
                            start <= end,
                        }

                        let v = ::linspace(start, end, n).rev().collect::<Vec<_>>();
                        let mut spaces = v.windows(2).map(|w| w[1] - w[0]);

                        test!(match spaces.next() {
                            None => true,
                            Some(first) => spaces.all(|space| space.is_close(first))
                        })
                    }

                    // Check that `linspace(..).rev()` produces a monotonically decreasing sequence
                    #[quickcheck]
                    fn monotonic(start: $ty, end: $ty, n: usize) -> TestResult {
                        enforce! {
                            start <= end,
                        }

                        let v = ::linspace(start, end, n).rev().collect::<Vec<_>>();

                        test!(v.windows(2).all(|w| w[1] <= w[0]))
                    }

                    // Check that `linspace(_, _, n).rev()` yields exactly `n` numbers
                    #[quickcheck]
                    fn size(start: $ty, end: $ty, n: usize) -> TestResult {
                        enforce! {
                            start <= end,
                        }

                        test!(::linspace(start, end, n).rev().count() == n)
                    }
                }

                // Check that `linspace(..)` yields evenly spaced numbers
                #[quickcheck]
                fn evenly_spaced(start: $ty, end: $ty, n: usize) -> TestResult {
                    use test::IsClose;

                    enforce! {
                        start <= end,
                    }

                    let v = ::linspace(start, end, n).collect::<Vec<_>>();
                    let mut spaces = v.windows(2).map(|w| w[1] - w[0]);

                    test!(match spaces.next() {
                        None => true,
                        Some(first) => spaces.all(|space| space.is_close(first))
                    })
                }

                // Check that `linspace(..)` produces a monotonic increasing sequence
                #[quickcheck]
                fn monotonic(start: $ty, end: $ty, n: usize) -> TestResult {
                    enforce! {
                        start <= end,
                    }

                    let v = ::linspace(start, end, n).collect::<Vec<_>>();

                    test!(v.windows(2).all(|w| w[1] >= w[0]))
                }

                // Check that `linspace(_, _, n)` yields exactly `n` numbers
                #[quickcheck]
                fn size(start: $ty, end: $ty, n: usize) -> TestResult {
                    enforce! {
                        start <= end,
                    }

                    test!(::linspace(start, end, n).count() == n)
                }
            }

            mod logspace {
                use quickcheck::TestResult;

                mod rev {
                    use quickcheck::TestResult;

                    // Check that `logspace(..).rev()` yields evenly spaced numbers
                    #[quickcheck]
                    fn evenly_spaced(start: $ty, end: $ty, n: usize) -> TestResult {
                        use test::IsClose;

                        enforce! {
                            start > 0.,
                            start <= end,
                        }

                        let v = ::logspace(start, end, n).rev().collect::<Vec<_>>();
                        let mut spaces = v.windows(2).map(|w| {
                            w[1].ln() - w[0].ln()
                        });

                        test!(match spaces.next() {
                            None => true,
                            Some(first) => spaces.all(|space| space.is_close(first))
                        })
                    }

                    // Check that `logspace(..).rev()` produces a monotonically decreasing sequence
                    #[quickcheck]
                    fn monotonic(start: $ty, end: $ty, n: usize) -> TestResult {
                        enforce! {
                            start > 0.,
                            start <= end,
                        }

                        let v = ::logspace(start, end, n).rev().collect::<Vec<_>>();

                        test!(v.windows(2).all(|w| w[1] <= w[0]))

                    }

                    // Check that `logspace(_, _, n).rev()` yields exactly `n` numbers
                    #[quickcheck]
                    fn size(start: $ty, end: $ty, n: usize) -> TestResult {
                        enforce! {
                            start > 0.,
                            start <= end,
                        }

                        test!(::logspace(start, end, n).count() == n)
                    }
                }

                // Check that `logspace(..)` yields evenly spaced numbers
                #[quickcheck]
                fn evenly_spaced(start: $ty, end: $ty, n: usize) -> TestResult {
                    use test::IsClose;

                    enforce! {
                        start > 0.,
                        start <= end,
                    }

                    let v = ::logspace(start, end, n).collect::<Vec<_>>();
                    let mut spaces = v.windows(2).map(|w| {
                        w[1].ln() - w[0].ln()
                    });

                    test!(match spaces.next() {
                        None => true,
                        Some(first) => spaces.all(|space| space.is_close(first))
                    })
                }

                // Check that `logspace(..)` produces a monotonically increasing sequence
                #[quickcheck]
                fn monotonic(start: $ty, end: $ty, n: usize) -> TestResult {
                    enforce! {
                        start > 0.,
                        start <= end,
                    }

                    let v = ::logspace(start, end, n).collect::<Vec<_>>();

                    test!(v.windows(2).all(|w| w[1] >= w[0]))
                }

                // Check that `logspace(_, _, n)` yields exactly `n` numbers
                #[quickcheck]
                fn size(start: $ty, end: $ty, n: usize) -> TestResult {
                    enforce! {
                        start > 0.,
                        start <= end,
                    }

                    test!(::logspace(start, end, n).count() == n)
                }
            }
        })+
    }
}

float!(f32, f64);
