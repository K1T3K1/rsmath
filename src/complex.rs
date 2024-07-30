use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy,
{
    #[inline]
    pub fn add(&self, operand: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.re + operand.re,
            im: self.im + operand.im,
        }
    }

    #[inline]
    pub fn substract(&self, operand: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.re - operand.re,
            im: self.im - operand.im,
        }
    }

    #[inline]
    pub fn multiply(&self, operand: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.re * operand.re - self.im * operand.im,
            im: self.im * operand.re + self.re * operand.im,
        }
    }

    #[inline]
    pub fn divide(&self, operand: &Complex<T>) -> Complex<T> {
        let divisor = operand.re * operand.re + operand.im * operand.im;
        Complex {
            re: (self.re * operand.re + self.im * operand.im) / divisor,
            im: (self.im * operand.re - self.re * operand.im) / divisor,
        }
    }

    #[inline]
    pub fn pow(&self, n: i32) -> Complex<T> {
        match n {
            x if x == 1 => self.clone(),
            _ => self.multiply(&self.pow(n - 1)),
        }
    }
}

//  TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let cnum = Complex { re: 5.0, im: 7.0 };
        let op = Complex { re: 7.0, im: 42.0 };
        let res = cnum.add(&op);
        assert_eq!(res, Complex { re: 12.0, im: 49.0 });
    }

    #[test]
    fn substract_test() {
        let cnum = Complex { re: 5.0, im: 7.0 };
        let op = Complex { re: 7.0, im: 42.0 };
        let res = cnum.substract(&op);
        assert_eq!(
            res,
            Complex {
                re: -2.0,
                im: -35.0
            }
        );
    }

    #[test]
    fn multiply_test() {
        let cnum = Complex { re: 5.0, im: 7.0 };
        let op = Complex { re: 7.0, im: 42.0 };
        let res = cnum.multiply(&op);
        assert_eq!(
            res,
            Complex {
                re: -259.0,
                im: 259.0
            }
        );
    }

    #[test]
    fn divide_test() {
        let cnum = Complex { re: 10.0, im: 10.0 };
        let op = Complex { re: 5.0, im: 5.0 };
        let res = cnum.divide(&op);
        assert_eq!(res, Complex { re: 2.0, im: 0.0 });
    }

    #[test]
    fn pow_test() {
        let cnum = Complex { re: 10.0, im: 10.0 };
        let res = cnum.pow(2);
        let comp = cnum.multiply(&cnum.clone());
        assert_eq!(res, comp);
    }
}
