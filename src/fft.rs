use crate::complex::Complex;
use num::{FromPrimitive, Num, ToPrimitive};
use rust_decimal::{Decimal, MathematicalOps};
use std::f64::consts::TAU;

pub fn fft<T: Num + FromPrimitive + ToPrimitive + Copy>(x: Vec<Complex<T>>) -> Vec<Complex<T>> {
    let length = x.len();
    (0..length)
        .map(|k| {
            x.iter().enumerate().fold(
                Complex {
                    re: T::zero(),
                    im: T::zero(),
                },
                |acc, (n, xn)| acc.add(&fft_fold(xn.clone(), n, k, length)),
            )
        })
        .collect()
}

fn fft_fold<T: Num + FromPrimitive + ToPrimitive + Copy>(
    xn: Complex<T>,
    n: usize,
    k: usize,
    length: usize,
) -> Complex<T> {
    let theta = (Decimal::from_f64(-TAU).unwrap()
        * Decimal::from_usize(n).unwrap()
        * Decimal::from_usize(k).unwrap())
        / Decimal::from_usize(length).unwrap();

    let sin = theta.sin();
    let cos = theta.cos();

    xn.multiply(&Complex {
        re: T::from_f64(cos.to_f64().unwrap()).unwrap(),
        im: T::from_f64(sin.to_f64().unwrap()).unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    #[rustfmt::skip]
    fn fft_test() {
        let x = vec![
            Complex { re: dec!(-2.0), im: dec!(4.0)},
            Complex { re: dec!(5.0), im: dec!(-5.0)},
            Complex { re: dec!(10.0), im: dec!(4.0)},
            Complex { re: dec!(-1.0), im: dec!(-9.0)},
            Complex { re: dec!(-8.0), im: dec!(3.0)},
            Complex { re: dec!(9.0), im: dec!(-5.0)},
            Complex { re: dec!(-8.0), im: dec!(-4.0)},
            Complex { re: dec!(3.0), im: dec!(-7.0)},
            Complex { re: dec!(-10.0), im: dec!(1.0)},
            Complex { re: dec!(-8.0), im: dec!(1.0)},
        ];
        let res = vec![
            Complex { re: dec!(-10.0), im: dec!(-17.0) }, 
            Complex { re: dec!(0.438028706609127207686783084), im: dec!(-9.795806641746903646163790450) }, 
            Complex { re: dec!(-9.914165332234708315373566167), im: dec!(-20.116478800684224607672419101) }, 
            Complex { re: dec!(-13.035915674137241776939650750), im: dec!(-10.021476573746968938491371349) }, 
            Complex { re: dec!(10.592866020451171869252867666), im: dec!(14.830046144997133384655161799) }, 
            Complex { re: dec!(-25.999999999999822084726981713), im: dec!(33.000000000000239569080499873) }, 
            Complex { re: dec!(35.385847742996760553879301499), im: dec!(-15.538250077575895023017257302) }, 
            Complex { re: dec!(-13.762458202143288146192518416), im: dec!(-4.103135223419962323146533148) }, 
            Complex { re: dec!(8.935451568518820038505735332), im: dec!(32.824682733009841069310323598) }, 
            Complex { re: dec!(-2.639654830564728830818952248), im: dec!(35.920418439292635084525885953) }
        ];
      
        let fft_x = fft(x);
        for i in 0..fft_x.len() {
            assert!((fft_x[i].re - res[i].re).abs().to_f64().unwrap() < f64::EPSILON);
            assert!((fft_x[i].im - res[i].im).abs().to_f64().unwrap() < f64::EPSILON);
        }
    }
}
