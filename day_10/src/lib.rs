use std::fmt;

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Clone + fmt::Display> Matrix<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Matrix<T> {
        let data = vec![value; rows * cols];
        Matrix { data, rows, cols }
    }
    pub fn at(&self, row_index: usize, col_index: usize) -> &T {
        let index = row_index * self.cols + col_index;
        &self.data[index]
    }
    pub fn at_mut(&mut self, row_index: usize, col_index: usize) -> &T {
        let index = row_index * self.cols + col_index;
        &mut self.data[index]
    }

    pub fn set_value(&mut self, value: T, row_index: usize, col_index: usize) {
        let index = row_index * self.cols + col_index;
        self.data[index] = value;
    }

    pub fn get_submatrix(
        &self,
        row_o: usize,
        row_l: usize,
        col_o: usize,
        col_l: usize,
    ) -> Matrix<T> {
        let mut submatrix: Matrix<T> =
            Matrix::new(row_l - row_o, col_l - col_o, self.at(0, 0).clone());
        for row in row_o..row_l {
            for col in col_o..col_l {
                submatrix.set_value(self.at(row, col).clone(), row - row_o, col - col_o);
            }
        }
        submatrix
    }

    pub fn get_row_vector(&self, row: usize, col_o: usize, col_l: usize) -> Vec<T> {
        let mut row_vector: Vec<T> = Vec::with_capacity(col_l - col_o);
        for col in col_o..col_l {
            let value = self.at(row, col).clone();
            row_vector.push(value);
        }
        row_vector
    }

    pub fn get_col_vector(&self, col: usize, row_o: usize, row_l: usize) -> Vec<T> {
        let mut col_vector: Vec<T> = Vec::with_capacity(row_l - row_o);
        for row in row_o..row_l {
            let value = self.at(row, col).clone();
            col_vector.push(value);
        }
        col_vector
    }

    pub fn insert_row(&mut self, row: usize, vector: Vec<T>) {
        let index = row * self.cols;
        if vector.len() != self.cols {
            panic!("Vector length must be equal to number of columns");
        }
        self.data.splice(index..index, vector);
        self.rows += 1;
    }

    pub fn insert_col(&mut self, col: usize, vector: Vec<T>) {
        if vector.len() != self.rows {
            panic!("Vector length must be equal to number of rows");
        }
        for (i, value) in vector.iter().enumerate() {
            let index = i * (self.cols + 1) + col;
            let value = value.clone();
            self.data.insert(index, value);
        }
        self.cols += 1;
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", *self.at(row, col))?;
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_row() {
        let mut matrix = Matrix::new(3, 3, 0);
        let row_to_insert = vec![1, 1, 1];
        matrix.insert_row(1, row_to_insert.clone());

        assert_eq!(matrix.rows, 4); // Check if rows increased

        for col in 0..matrix.cols {
            assert_eq!(*matrix.at(1, col), row_to_insert[col]); // Check inserted row
        }
    }

    #[test]
    fn test_insert_col() {
        let mut matrix = Matrix::new(3, 3, 0);
        let col_to_insert = vec![1, 1, 1];
        matrix.insert_col(1, col_to_insert.clone());

        assert_eq!(matrix.cols, 4); // Check if cols increased

        for row in 0..matrix.rows {
            assert_eq!(*matrix.at(row, 1), col_to_insert[row]); // Check inserted col
        }
    }
}
