use num::{FromPrimitive, Zero};
use std::{
    iter::{Product, Sum},
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
pub struct Matrix2D<T> {
    data: Vec<Vec<T>>,
    width: usize,
}

#[derive(Debug, PartialEq)]
pub enum Matrix2DError {
    NotMultiplicable,
    NotAdditive,
    NotSquare,
    EmptyMatrix,
    EmptyRow,
    InconsistentRowLength,
}

impl<T> Index<usize> for Matrix2D<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Matrix2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clone
        + Copy
        + Zero
        + Default
        + FromPrimitive
        + Sum
        + Product
        + std::fmt::Debug,
{
    pub fn new(data: Vec<Vec<T>>) -> Result<Matrix2D<T>, Matrix2DError> {
        if data.is_empty() {
            return Err(Matrix2DError::EmptyMatrix);
        }

        let width = data[0].len();

        if width == 0 {
            return Err(Matrix2DError::EmptyRow);
        }

        if !data.iter().all(|row| row.len() == width) {
            return Err(Matrix2DError::InconsistentRowLength);
        }

        Ok(Matrix2D { width, data })
    }

    pub fn diag(size: usize, value: T) -> Self {
        let mut data = vec![vec![T::zero(); size]; size];

        for i in 0..size {
            data[i][i] = value;
        }

        Matrix2D::new(data).unwrap()
    }

    pub fn mul(&self, operand: &Matrix2D<T>) -> Result<Matrix2D<T>, Matrix2DError> {
        match self.is_multiplicable(&operand) {
            false => return Err(Matrix2DError::NotMultiplicable),
            _ => {}
        }

        let mut result: Vec<Vec<T>> = vec![Vec::new(); self.data.len()];
        for (idx, r) in self.data.iter().enumerate() {
            for (jdx, _) in operand[idx].iter().enumerate() {
                result[idx].push(
                    r.iter()
                        .fold(T::zero(), |acc, x| acc + (*x * operand[idx][jdx])),
                );
            }
        }

        Ok(Matrix2D {
            width: result[0].len(),
            data: result,
        })
    }

    pub fn add(&self, operand: &Matrix2D<T>) -> Result<Matrix2D<T>, Matrix2DError> {
        match self.is_additive(operand) {
            false => return Err(Matrix2DError::NotAdditive),
            _ => {}
        }

        Ok(Matrix2D::new(
            self.data
                .iter()
                .zip(operand.data.iter())
                .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(a, b)| *a + *b).collect())
                .collect(),
        )
        .unwrap())
    }

    pub fn substract(&self, operand: &Matrix2D<T>) -> Result<Matrix2D<T>, Matrix2DError> {
        match self.is_additive(operand) {
            false => return Err(Matrix2DError::NotAdditive),
            _ => {}
        }

        Ok(Matrix2D::new(
            self.data
                .iter()
                .zip(operand.data.iter())
                .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(a, b)| *a - *b).collect())
                .collect(),
        )
        .unwrap())
    }

    pub fn det(&self) -> Result<T, Matrix2DError> {
        match self.is_square() {
            false => return Err(Matrix2DError::NotSquare),
            _ => {}
        }

        if self.width == 1 {
            return Ok(self[0][0]);
        }

        let (_, u) = self.lu_decomposition().unwrap();

        Ok(u.data
            .iter()
            .enumerate()
            .map(|(idx, row)| row[idx])
            .product())
    }

    pub fn lu_decomposition(&self) -> Result<(Matrix2D<T>, Matrix2D<T>), Matrix2DError> {
        match self.is_square() {
            false => return Err(Matrix2DError::NotSquare),
            _ => {}
        }

        let mut l = Matrix2D::diag(self.width, T::from_i32(1).unwrap());
        let mut u = Matrix2D::diag(self.width, T::from_i32(0).unwrap());
        for i in 0..self.width {
            for j in 0..self.width {
                if i <= j {
                    let sum = (0..i).map(|k| l[i][k] * u[k][j]).sum();
                    u[i][j] = self[i][j] - sum;
                } else {
                    let sum = (0..j).map(|k| l[i][k] * u[k][j]).sum();
                    l[i][j] = (self[i][j] - sum) / u[j][j];
                }
            }
        }

        for i in 0..self.width {
            l[i][i] = T::from_u8(1).unwrap();
        }

        Ok((l, u))
    }

    #[inline]
    fn is_multiplicable(&self, operand: &Matrix2D<T>) -> bool {
        self.width == operand.data.len()
    }

    #[inline]
    fn is_additive(&self, operand: &Matrix2D<T>) -> bool {
        self.width == operand.width && self.data.len() == operand.data.len()
    }

    fn is_square(&self) -> bool {
        self.data.len() == self[0].len()
    }
}

//  TESTS
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn new_test() {
        assert!(Matrix2D::new(vec![vec![5, 15, 25], vec![8, 85, 25], vec![85, 25, 35]]).is_ok())
    }

    #[test]
    fn new_err_emptymatrix_test() {
        let mat: Result<Matrix2D<i32>, Matrix2DError> = Matrix2D::new(vec![]);
        assert_eq!(mat, Err(Matrix2DError::EmptyMatrix));
    }

    #[test]
    fn new_err_emptyrow_test() {
        let mat: Result<Matrix2D<i32>, Matrix2DError> = Matrix2D::new(vec![vec![]]);
        assert_eq!(mat, Err(Matrix2DError::EmptyRow));
    }

    #[test]
    fn new_err_inconsistent_test() {
        let mat: Result<Matrix2D<i32>, Matrix2DError> = Matrix2D::new(vec![vec![25], vec![58, 25]]);
        assert_eq!(mat, Err(Matrix2DError::InconsistentRowLength));
    }

    #[test]
    fn matmul_test() {
        let res = Matrix2D::new(vec![vec![2, 4], vec![3, 6]]).unwrap();
        let mult_res = Matrix2D::new(vec![vec![1, 0], vec![0, 1]])
            .unwrap()
            .mul(&Matrix2D::new(vec![vec![2, 4], vec![3, 6]]).unwrap());
        assert_eq!(res, mult_res.unwrap());
    }

    #[test]
    fn matmul_err_test() {
        assert_eq!(
            Matrix2D::new(vec![vec![7]])
                .unwrap()
                .mul(&Matrix2D::new(vec![vec![1], vec![2]]).unwrap()),
            Err(Matrix2DError::NotMultiplicable)
        );
    }
    #[test]
    fn matadd_test() {
        let res = Matrix2D::new(vec![vec![20, 20], vec![20, 20]]).unwrap();
        let add_res = Matrix2D::new(vec![vec![7, 11], vec![3, 15]])
            .unwrap()
            .add(&Matrix2D::new(vec![vec![13, 9], vec![17, 5]]).unwrap());
        assert_eq!(res, add_res.unwrap());
    }

    #[test]
    fn matadd_err_test() {
        assert_eq!(
            Matrix2D::new(vec![vec![7]])
                .unwrap()
                .add(&Matrix2D::new(vec![vec![1], vec![2]]).unwrap()),
            Err(Matrix2DError::NotAdditive)
        );
    }

    #[test]
    fn matsquare_err_test() {
        assert_eq!(
            Matrix2D::new(vec![vec![2, 3, 4], vec![2, 7, 9]])
                .unwrap()
                .det(),
            Err(Matrix2DError::NotSquare)
        );
    }

    use ::rust_decimal_macros::dec;
    #[test]
    fn det_test() {
        assert_eq!(
            Matrix2D::new(vec![vec![dec!(5.0), dec!(7.0)], vec![dec!(7.0), dec!(9.0)]])
                .unwrap()
                .det(),
            Ok(dec!(-4.0))
        )
    }

    #[test]
    fn lu_test() {
        assert_eq!(
            Matrix2D::new(vec![vec![dec!(5.0), dec!(7.0)], vec![dec!(7.0), dec!(9.0)]])
                .unwrap()
                .lu_decomposition().unwrap(),
            (
                Matrix2D::new(vec![vec![dec!(1.0), dec!(0.0)], vec![dec!(1.4), dec!(1.0)]]).unwrap(),
                Matrix2D::new(vec![vec![dec!(5.0), dec!(7.0)], vec![dec!(0.0), dec!(-0.8)]]).unwrap()
            )
        )
    }
}
