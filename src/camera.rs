use std::fmt;
use std::f64::consts::PI;
use crate::arithmetic::*;
use crate::matrix::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct Camera
{
    pub hsize: u16,
    pub vsize: u16,
    pub field_of_view: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    pub transform: Matrix,
}

impl Camera
{
    pub fn new(hsize: u16, vsize: u16, field_of_view: f64) -> Self
    {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);
        let half_width;
        let half_height;
        if aspect >= 1.0
        {
            half_width = half_view;
            half_height = half_view / aspect;
        }
        else
        {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / f64::from(hsize);
        Camera{hsize, vsize, field_of_view,
            half_width, half_height, pixel_size,
            transform: Matrix::identity(4)}
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_camera_feature()
    {
        // p.101 Scenario: Constructing a camera
        let c1 = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c1.hsize, 160);
        assert_eq!(c1.vsize, 120);
        assert!(fuzzy_equal(c1.field_of_view, PI / 2.0));
        assert_eq!(c1.transform, Matrix::identity(4));

        // p.101 Scenario: The pixel size for a horizontal canvas
        let c2 = Camera::new(200, 125, PI / 2.0);
        assert!(fuzzy_equal(c2.pixel_size, 0.01));

        // p.101 Scenario: The pixel size for a vertical canvas
        let c3 = Camera::new(125, 200, PI / 2.0);
        assert!(fuzzy_equal(c3.pixel_size, 0.01));
    }
}
