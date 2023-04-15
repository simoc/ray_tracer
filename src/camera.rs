use std::fmt;
use std::f64::consts::PI;
use crate::arithmetic::*;
use crate::canvas::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;
use crate::world::*;

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

    pub fn ray_for_pixel(&self, px: u16, py: u16) -> Ray
    {
        // the offset from the edge of the canvas to the pixel's center.
        let xoffset = (f64::from(px) + 0.5) * self.pixel_size;
        let yoffset = (f64::from(py) + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the camera is at z=-1).
        let inverse = self.transform.inverse();
        let pixel = inverse.multiply_tuple(create_point(world_x, world_y, -1.0));
        let origin = inverse.multiply_tuple(create_point(0.0, 0.0, 0.0));
        let direction = pixel.sub(origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas
    {
        let mut image = Canvas::new(self.hsize.into(), self.vsize.into());
        for y in 0..self.vsize - 1
        {
            for x in 0..self.hsize - 1
            {
                let ray = self.ray_for_pixel(x.into(), y.into());
                let color = world.color_at(ray);
                image.write_pixel(x.into(), y.into(), color);
            }
        }
        image
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

        // p.103 Scenario: Constructing a ray through the center of the canvas
        let c4 = Camera::new(201, 101, PI / 2.0);
        let r4 = c4.ray_for_pixel(100, 50);
        assert_eq!(r4.origin, create_point(0.0, 0.0, 0.0));
        assert_eq!(r4.direction, create_vector(0.0, 0.0, -1.0));

        // p.103 Scenario: Constructing a ray through a corner of the canvas
        let c5 = Camera::new(201, 101, PI / 2.0);
        let r5 = c5.ray_for_pixel(0, 0);
        assert_eq!(r5.origin, create_point(0.0, 0.0, 0.0));
        assert_eq!(r5.direction, create_vector(0.66519, 0.33259, -0.66851));

        // p.103 Scenario: Constructing a ray when the camera is transformed
        let mut c6 = Camera::new(201, 101, PI / 2.0);
        c6.transform = Matrix::rotation_y(PI / 4.0).multiply(&Matrix::translation(0.0, -2.0, 5.0));
        let r6 = c6.ray_for_pixel(100, 50);
        assert_eq!(r6.origin, create_point(0.0, 2.0, -5.0));
        let sqrt2 = 2.0_f64.sqrt();
        assert_eq!(r6.direction, create_vector(sqrt2 / 2.0, 0.0, -sqrt2 / 2.0));

        // p.104 Scenario: Rendering a world with a camera
        let world7 = World::default_world();
        let mut c7 = Camera::new(11, 11, PI / 2.0);
        let from7 = create_point(0.0, 0.0, -5.0);
        let to7 = create_point(0.0, 0.0, 0.0);
        let up7 = create_point(0.0, 1.0, 0.0);
        c7.transform = Matrix::view_transform(from7, to7, up7);
        let image7 = c7.render(world7);
        assert_eq!(image7.pixel_at(5, 5), create_color(0.38066, 0.47583, 0.2855));
    }
}
