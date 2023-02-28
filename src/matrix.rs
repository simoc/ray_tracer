use std::fmt;
use crate::tuple::*;
use crate::arithmetic::*;

#[derive(Debug)]
pub struct Matrix
{
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<Vec<f64>>,
}

impl Matrix
{
    pub fn new(rows: usize, columns: usize, cell_values: &Vec<f64>) -> Self
    {
        if rows * columns != cell_values.len()
        {
            panic!("Wrong number of elements for a {}x{} matrix: {}",
                rows, columns, cell_values.len());
        }
        let mut cell_index = 0;
        let mut cells = Vec::with_capacity(rows);
        for _ in 0..rows
        {
            let mut row = Vec::with_capacity(columns);
            for _ in 0..columns
            {
                row.push(cell_values[cell_index]);
                cell_index = cell_index + 1;
            }
            cells.push(row);
        }
        Matrix{rows: rows, columns: columns, cells: cells}
    }

    pub fn at(&self, y : usize, x: usize) -> f64
    {
        self.cells[y][x]
    }

    pub fn multiply(&self, b: &Matrix) -> Matrix
    {
        let mut cells = Vec::with_capacity(self.rows);
        for y in 0..self.rows
        {
            let mut row = Vec::with_capacity(self.columns);
            for x in 0..self.columns
            {
                let mut total = 0.0;
                for i in 0..self.columns
                {
                    total = total + (self.cells[y][i] * b.cells[i][x]);
                }
                row.push(total);
            }
            cells.push(row);
        }
        Matrix{rows: self.rows, columns: b.columns, cells: cells}
    }

    pub fn multiply_tuple(&self, b: Tuple) -> Tuple
    {
        let bv = b.get_vec();
        let mut mv = Vec::new();
        for y in 0..self.rows
        {
            let mut total = 0.0;
            for x in 0..self.columns
            {
                total = total + (self.cells[y][x] * bv[x]);
            }
            mv.push(total);
        }
        create_tuple(mv[0], mv[1], mv[2], mv[3])
    }

    pub fn identity(dimension: usize) -> Matrix
    {
        let mut cells = Vec::with_capacity(dimension);
        for y in 0..dimension
        {
            let mut row = Vec::with_capacity(dimension);
            for x in 0..dimension
            {
                if x == y
                {
                    row.push(1.0);
                }
                else
                {
                    row.push(0.0);
                }
            }
            cells.push(row);
        }
        Matrix{rows: dimension, columns: dimension, cells: cells}
    }

    pub fn transpose(&self) -> Matrix
    {
        let mut cells = Vec::with_capacity(self.columns);
        for x in 0..self.columns
        {
            let mut row = Vec::with_capacity(self.rows);
            for y in 0..self.rows
            {
                row.push(self.at(y, x));
            }
            cells.push(row);
        }
        Matrix{rows: self.columns, columns: self.rows, cells: cells}
    }

    pub fn determinant(&self) -> f64
    {
        if self.rows == 2 && self.columns == 2
        {
            return (self.at(0, 0) * self.at(1, 1)) - (self.at(0, 1) * self.at(1, 0));
        }
        let mut det = 0.0;
        for x in 0..self.columns
        {
            let n = self.at(0, x);
            let cofactor = self.cofactor(0, x);
            det = det + (n * cofactor);
        }
        det
    }

    pub fn submatrix(&self, omit_row: usize, omit_column: usize) -> Matrix
    {
        let mut cells = Vec::with_capacity(self.rows - 1);
        for y in 0..self.rows
        {
            let mut row = Vec::with_capacity(self.columns - 1);
            for x in 0..self.columns
            {
                if y != omit_row && x != omit_column
                {
                    row.push(self.at(y, x));
                }
            }
            if row.len() > 0
            {
                cells.push(row);
            }
        }
        Matrix{rows: self.rows - 1, columns: self.columns - 1, cells: cells}
    }

    pub fn minor(&self, row: usize, column: usize) -> f64
    {
        let submatrix = self.submatrix(row, column);
        submatrix.determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64
    {
        let minor = self.minor(row, column);
        if ((row + column) % 2) == 0
        {
            minor
        }
        else
        {
            -minor
        }
    }

    pub fn invertible(&self) -> bool
    {
        !fuzzy_equal(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Matrix
    {
        let m_det = self.determinant();
        if fuzzy_equal(m_det, 0.0)
        {
            panic!("Matrix is not invertible");
        }

        let mut m2 = Matrix::identity(self.rows);
        for y in 0..self.rows
        {
            for x in 0..self.columns
            {
                let c = self.cofactor(y, x);
                m2.cells[x][y] = c / m_det;
            }
        }
        m2
    }
}

impl fmt::Display for Matrix
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut layout = String::new();
        for y in 0..self.rows
        {
            for x in 0..self.columns
            {
                layout.push_str("| ");
                let cell = format!("{:>8.4} ", self.at(y, x));
                layout.push_str(&cell);
            }
            layout.push_str("|\n");
        }
        write!(f, "{}", layout)
    }
}

impl PartialEq for Matrix
{
    fn eq(&self, other: &Self) -> bool
    {
        if self.rows != other.rows || self.columns != other.columns
        {
            return false;
        }
        for y in 0..self.rows
        {
            for x in 0..self.columns
            {
                if !fuzzy_equal(self.cells[y][x], other.cells[y][x])
                {
                    return false;
                }
            }
        }
        return true;
    }
}

pub fn matrix_from(cell_values: &str) -> Matrix
{
    let mut columns = 0;
    let mut cells = Vec::new();
    let without_separators = cell_values.replace("|", " ");
    let lines = without_separators.lines();
    for line in lines
    {
        let mut row = Vec::new();
        for n in line.split_whitespace()
        {
            row.push(n.parse::<f64>().unwrap());
        }
        columns = row.len();
        cells.push(row);
    }
    Matrix{rows: cells.len(), columns: columns, cells: cells}
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_matrices_feature()
    {
        // p.26 Scenario: Constructing and inspecting a 4x4 matrix
        let m1 = Matrix::new(4, 4, &vec![1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5]);
        assert_eq!(m1.rows, 4);
        assert_eq!(m1.columns, 4);
        assert!(fuzzy_equal(m1.at(0, 0), 1.0));
        assert!(fuzzy_equal(m1.at(1, 0), 5.5));
        assert!(fuzzy_equal(m1.at(1, 2), 7.5));
        assert!(fuzzy_equal(m1.at(2, 2), 11.0));
        assert!(fuzzy_equal(m1.at(3, 0), 13.5));
        assert!(fuzzy_equal(m1.at(3, 2), 15.5));

        // p.27 Scenario: A 2x2 matrix ought to be representable
        let m2 = Matrix::new(2, 2, &vec![-3.0, 5.0,
            1.0, -2.0]);
        assert!(fuzzy_equal(m2.at(0, 0), -3.0));
        assert!(fuzzy_equal(m2.at(0, 1), 5.0));
        assert!(fuzzy_equal(m2.at(1, 0), 1.0));
        assert!(fuzzy_equal(m2.at(1, 1), -2.0));

        // p.27 Scenario: A 3x3 matrix ought to be representable
        let m3 = Matrix::new(3, 3, &vec![-3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0]);
        assert!(fuzzy_equal(m3.at(0, 0), -3.0));
        assert!(fuzzy_equal(m3.at(1, 1), -2.0));
        assert!(fuzzy_equal(m3.at(2, 2), 1.0));

        // p.27 Scenario: Matrix equality with identical matrices
        let v4 = vec![1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0];
        let m4 = Matrix::new(4, 4, &v4);
        let m5 = Matrix::new(4, 4, &v4);
        assert_eq!(m4, m5);

        // p.27 Scenario: Matrix equality with different matrices
        let m6 = Matrix::new(4, 4, &v4);
        let v7 = vec![2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0];
        let m7 = Matrix::new(4, 4, &v7);
        assert_ne!(m6, m7);

        // p.28 Scenario: Multiplying two matrices
        let m8 = Matrix::new(4, 4, &v4);
        let v9 = vec![-2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0];
        let m9 = Matrix::new(4, 4, &v9);
        let m10 = m8.multiply(&m9);
        let m11 = Matrix::new(4, 4, &vec![20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0]);
        assert_eq!(m10, m11);

        // p.28 Scenario: A matrix multiplied by a tuple
        let m12 = Matrix::new(4, 4, &vec![1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0]);
        let t13 = m12.multiply_tuple(create_tuple(1.0, 2.0, 3.0, 1.0));
        assert!(crate::tuple::equal(t13, create_tuple(18.0, 24.0, 33.0, 1.0)));

        // p.32 Scenario: Multiplying a matrix by the identity matrix
        let m14 = Matrix::new(4, 4, &vec![0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0]);
        let m15 = m14.multiply(&Matrix::identity(4));
        assert_eq!(m14, m15);

        // p.33 Scenario: Transposing a matrix
        let m16 = Matrix::new(4, 4, &vec![0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0]);
        let m17 = m16.transpose();
        assert_eq!(m17, Matrix::new(4, 4, &vec![0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0]));

        // p.33 Scenario: Transposing the identity matrix
        assert_eq!(Matrix::identity(4), Matrix::identity(4).transpose());

        // p.34 Scenario: Calculating the determinant of a 2x2 matrix
        let d1 = Matrix::new(2, 2, &vec![1.0, 5.0, -3.0, 2.0]).determinant();
        assert!(fuzzy_equal(d1, 17.0));

        // p.35 Scenario: A submatrix of a 3x3 matrix is a 2x2 matrix
        let m18 = Matrix::new(3, 3, &vec![1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0]);
        let m19 = Matrix::new(2, 2, &vec![-3.0, 2.0, 0.0, 6.0]);
        assert_eq!(m18.submatrix(0, 2), m19);

        // p.35 Scenario: A submatrix of a 4x4 matrix is a 3x3 matrix
        let m20 = Matrix::new(4, 4, &vec![-6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0]);
        let m21 = Matrix::new(3, 3, &vec![-6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0]);
        assert_eq!(m20.submatrix(2, 1), m21);

        // p.35 Scenario: Calculating a minor of a 3x3 matrix
        let m22 = Matrix::new(3, 3, &vec![3.0, 5.0, 0.0,
            2.0, -1.0, 7.0,
            6.0, -1.0, 5.0]);
        assert!(fuzzy_equal(m22.minor(1, 0), 25.0));

        // p.36 Scenario: Calculating a cofactor of a 3x3 matrix
        let m23 = Matrix::new(3, 3, &vec![3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0]);
        assert!(fuzzy_equal(m23.cofactor(0, 0), -12.0));
        assert!(fuzzy_equal(m23.cofactor(1, 0), -25.0));

        // p.37 Scenario: Calculating the determinant of a 3x3 matrix
        let m24 = Matrix::new(3, 3, &vec![1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0]);
        assert!(fuzzy_equal(m24.cofactor(0, 0), 56.0));
        assert!(fuzzy_equal(m24.cofactor(0, 1), 12.0));
        assert!(fuzzy_equal(m24.cofactor(0, 2), -46.0));
        assert!(fuzzy_equal(m24.determinant(), -196.0));

        // p.37 Scenario: Calculating the determinant of a 4x4 matrix
        let m25 = Matrix::new(4, 4, &vec![-2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0]);
        assert!(fuzzy_equal(m25.cofactor(0, 0), 690.0));
        assert!(fuzzy_equal(m25.cofactor(0, 1), 447.0));
        assert!(fuzzy_equal(m25.cofactor(0, 2), 210.0));
        assert!(fuzzy_equal(m25.cofactor(0, 3), 51.0));
        assert!(fuzzy_equal(m25.determinant(), -4071.0));

        // p.39 Scenario: Testing an invertible matrix for invertibility
        let m26 = Matrix::new(4, 4, &vec![6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0]);
        assert!(m26.invertible());

        // p.39 Scenario: Testing an noninvertible matrix for invertibility
        let m27 = Matrix::new(4, 4, &vec![-4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0]);
        assert!(!m27.invertible());
    }

    #[test]
    fn test_matrices_feature_inverse()
    {
        // p.39 Scenario: Calculating the inverse of a matrix
        let m28 = Matrix::new(4, 4, &vec![-5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0]);
        let m29 = m28.inverse();
        assert!(fuzzy_equal(m28.determinant(), 532.0));
        assert!(fuzzy_equal(m28.cofactor(2, 3), -160.0));
        assert!(fuzzy_equal(m29.at(3, 2), -160.0 / 532.0));
        assert!(fuzzy_equal(m28.cofactor(3, 2), 105.0));
        assert!(fuzzy_equal(m29.at(2, 3), 105.0 / 532.0));
        let m30 = Matrix::new(4, 4, &vec![0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639]);
        assert_eq!(m29, m30);

        // p.41 Scenario: Calculating the inverse of another matrix
        let m31 = Matrix::new(4, 4, &vec![8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0]);
        let m32 = m31.inverse();
        let m33 = Matrix::new(4, 4, &vec![-0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308]);
        assert_eq!(m32, m33);

        // p.41 Scenario: Calculating the inverse of a third matrix
        let m34 = Matrix::new(4, 4, &vec![9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0]);
        let m35 = m34.inverse();
        let m36 = Matrix::new(4, 4, &vec![-0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333]);
        assert_eq!(m35, m36);
    }
}
