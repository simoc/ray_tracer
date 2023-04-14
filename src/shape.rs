use std::fmt;

use crate::sphere::*;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub enum Shape
{
    Sphere(Sphere),
}

impl Shape
{
    pub fn new_sphere(id: i32) -> Shape
    {
        Shape::Sphere(Sphere::new(id))
    }

    pub fn test_shape(id: i32) -> Shape
    {
        Self::new_sphere(id)
    }

    pub fn get_transform(&self) -> Matrix
    {
        match &self
        {
            Shape::Sphere(s) => s.get_local_transform(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix)
    {
        match self
        {
            Shape::Sphere(s) => s.set_local_transform(transform),
        }
    }

    pub fn get_material(&self) -> Material
    {
        match self
        {
            Shape::Sphere(s) => s.get_local_material(),
        }
    }

    pub fn set_material(&mut self, material: Material)
    {
        match self
        {
            Shape::Sphere(s) => s.set_local_material(material),
        }
    }

    pub fn intersect(&mut self, ray: Ray) -> Vec<f64>
    {
        match self
        {
            Shape::Sphere(s) => s.local_intersect(ray),
        }
    }

    pub fn get_saved_ray(&self) -> Ray
    {
        match self
        {
            Shape::Sphere(s) => s.local_get_saved_ray(),
        }
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple
    {
        match self
        {
            Shape::Sphere(s) => s.local_normal_at(world_point),
        }
    }
}

impl PartialEq for Shape
{
    fn eq(&self, other: &Self) -> bool
    {
        match self
        {
            Shape::Sphere(s1) =>
            {
                match other
                {
                    Shape::Sphere(s2) => s1.get_id() == s2.get_id(),
                }
            },
        }
    }
}

impl fmt::Display for Shape
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Shape::Sphere(s) => write!(f, "sphere {}", s.get_id()),
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_shape_feature()
    {
        // p.119 Scenario: The default transformation
        let s1 = Shape::test_shape(1);
        assert_eq!(s1.get_transform(), Matrix::identity(4));

        // p.119 Scenario: Assigning a transformation
        let mut s2 = Shape::test_shape(2);
        s2.set_transform(Matrix::translation(2.0, 3.0, 4.0));
        assert_eq!(s2.get_transform(), Matrix::translation(2.0, 3.0, 4.0));

        // p.119 Scenario: The default material
        let s3 = Shape::test_shape(3);
        assert_eq!(s3.get_material(), Material::new());

        // p.119 Scenario: Assigning a material
        let mut s4 = Shape::test_shape(4);
        let mut m4 = Material::new();
        m4.ambient = 1.0;
        s4.set_material(m4);
        assert_eq!(s4.get_material(), m4);

        // p.120 Scenario: Intersecting a scaled shape with a ray
        let r5 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let mut s5 = Shape::test_shape(5);
        s5.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let _xs5 = s5.intersect(r5);
        assert_eq!(s5.get_saved_ray().origin, create_point(0.0, 0.0, -2.5));
        assert_eq!(s5.get_saved_ray().direction, create_vector(0.0, 0.0, 0.5));
    }
}
