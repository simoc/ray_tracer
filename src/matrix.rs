use std::fmt;
use crate::tuple::*;

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
        let mut cell_index = 0;
        let mut cells = Vec::with_capacity(rows);
        for _ in 0..rows
        {
            let mut row = Vec::with_capacity(columns);
            for _ in 0..columns
            {
                if cell_index < cell_values.len()
                {
                    row.push(cell_values[cell_index]);
                    cell_index = cell_index + 1;
                }
                else
                {
                    row.push(0.0);
                }
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
        let mut cells = Vec::new();
        for y in 0..self.rows
        {
            let mut row = Vec::new();
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
        let mut cells = Vec::new();
        for y in 0..dimension
        {
            let mut row = Vec::new();
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
        let mut cells = Vec::new();
        for y in 0..self.rows
        {
            let mut row = Vec::new();
            for x in 0..self.columns
            {
                row.push(self.at(x, y));
            }
            cells.push(row);
        }
        Matrix{rows: self.columns, columns: self.rows, cells: cells}
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
    let cells = Vec::new();
    let mut columns = 0;
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
    }
}
