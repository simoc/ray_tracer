use std::fmt;
use crate::tuple::*;

pub struct Matrix
{
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<Vec<f64>>,
}

impl Matrix
{
    pub fn at(&self, y : usize, x: usize) -> f64
    {
        self.cells[y][x]
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

pub fn create_matrix(rows: usize, columns: usize, cell_values: Vec<f64>) -> Matrix
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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_matrices_feature()
    {
        // p.26 Scenario: Constructing and inspecting a 4x4 matrix
        let m1 = create_matrix(4, 4, vec![1.0, 2.0, 3.0, 4.0,
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
        let m2 = create_matrix(2, 2, vec![-3.0, 5.0,
            1.0, -2.0]);
        assert!(fuzzy_equal(m2.at(0, 0), -3.0));
        assert!(fuzzy_equal(m2.at(0, 1), 5.0));
        assert!(fuzzy_equal(m2.at(1, 0), 1.0));
        assert!(fuzzy_equal(m2.at(1, 1), -2.0));

        // p.27 Scenario: A 3x3 matrix ought to be representable
        let m3 = create_matrix(3, 3, vec![-3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0]);
        assert!(fuzzy_equal(m3.at(0, 0), -3.0));
        assert!(fuzzy_equal(m3.at(1, 1), -2.0));
        assert!(fuzzy_equal(m3.at(2, 2), 1.0));
    }
}
