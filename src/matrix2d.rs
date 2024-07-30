use num::Zero;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Matrix2D<T> {
    data: Vec<Vec<T>>,
    width: usize,
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
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy + Zero,
{
    pub fn new(data: Vec<Vec<T>>) -> Matrix2D<T> {
        Matrix2D {
            width: data[0].len(),
            data: data,
        }
    }

    pub fn mul(&self, operand: &Matrix2D<T>) -> Result<Matrix2D<T>, &'static str> {
        match self.is_multiplicable(&operand) {
            false => return Err("Matrices are not multiplicable"),
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

    pub fn add(&self, operand: &Matrix2D<T>) -> Result<Matrix2D<T>, &'static str> {
        match self.is_additive(operand) {
            false => return Err("Matrices are not additive"),
            _ => {}
        }

        Ok(Matrix2D::new(
            self.data
                .iter()
                .zip(operand.data.iter())
                .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(a, b)| *a + *b).collect())
                .collect(),
        ))
    }

    pub fn det(&self) -> Result<T, &'static str> {
        match self.is_square() {
            false => return Err("Matrice is not square"),
            _ => {}
        }

        if self.width == 1 {
            return Ok(self[0][0]);
        }

        // lacks impl

        todo!()
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
    fn matmul_test() {
        let res = Matrix2D::new(vec![vec![2, 4], vec![3, 6]]);
        let mult_res = Matrix2D::new(vec![vec![1, 0], vec![0, 1]])
            .mul(&Matrix2D::new(vec![vec![2, 4], vec![3, 6]]));
        assert_eq!(res, mult_res.unwrap());
    }

    #[test]
    fn matmul_err_test() {
        assert!(Matrix2D::new(vec![vec![7]])
            .mul(&Matrix2D::new(vec![vec![1], vec![2]]))
            .is_err());
    }
    #[test]
    fn matadd_test() {
        let res = Matrix2D::new(vec![vec![20, 20], vec![20, 20]]);
        let add_res = Matrix2D::new(vec![vec![7, 11], vec![3, 15]])
            .add(&Matrix2D::new(vec![vec![13, 9], vec![17, 5]]));
        assert_eq!(res, add_res.unwrap());
    }

    #[test]
    fn matadd_err_test() {
        assert!(Matrix2D::new(vec![vec![7]])
            .add(&Matrix2D::new(vec![vec![1], vec![2]]))
            .is_err());
    }
}
