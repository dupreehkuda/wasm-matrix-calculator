use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {
        let expected_size = rows * cols;
        if data.len() != expected_size {
            panic!("Invalid data size for matrix dimensions!");
        }

        Matrix { rows, cols, data }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        self.check_dimensions(other);

        let result_data: Vec<f64> = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a + b)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        }
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        self.check_dimensions(other);

        let result_data: Vec<f64> = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a - b)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        self.check_multiply_dimensions(other);

        let mut result_data = Vec::with_capacity(self.rows * other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get_element(i, k) * other.get_element(k, j);
                }
                result_data.push(sum);
            }
        }

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: result_data,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_array(&self) -> Vec<f64> {
        self.data.clone()
    }

    pub fn get_element(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    fn check_dimensions(&self, other: &Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrix dimensions do not match for addition!");
        }
    }

    fn check_multiply_dimensions(&self, other: &Matrix) {
        if self.cols != other.rows {
            panic!("Invalid matrix dimensions for multiplication!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let matrix1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let result = matrix1.add(&matrix2);

        assert_eq!(result.to_array(), vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_matrix_subtraction() {
        let matrix1 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let matrix2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

        let result = matrix1.subtract(&matrix2);

        assert_eq!(result.to_array(), vec![4.0, 4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_matrix_multiplication() {
        let matrix1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let result = matrix1.multiply(&matrix2);

        assert_eq!(result.to_array(), vec![19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn test_matrix_multiplication_2() {
        let matrix1 = Matrix::new(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let matrix2 = Matrix::new(2, 3, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);

        let result = matrix1.multiply(&matrix2);

        assert_eq!(result.to_array(), vec![27.0, 30.0, 33.0, 61.0, 68.0, 75.0, 95.0, 106.0, 117.0]);
    }
}