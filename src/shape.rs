use std::fmt;

use crate::cube::*;
use crate::sphere::*;
use crate::material::*;
use crate::matrix::*;
use crate::plane::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub enum ShapeSpecific
{
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

#[derive(Clone, Debug)]
pub struct Shape
{
    id: i32,
    transform: Matrix,
    material: Material,
    saved_ray: Ray,
    specific: ShapeSpecific,
}

impl Shape
{
    pub fn new_sphere(id: i32) -> Shape
    {
        let zero_point = create_point(0.0, 0.0, 0.0);
        let zero_vector = create_vector(0.0, 0.0, 0.0);

        Shape{id: id,
            transform: Matrix::identity(4),
            material: Material::new(),
            saved_ray: Ray::new(zero_point, zero_vector),
            specific: ShapeSpecific::Sphere(Sphere::new())}
    }

    pub fn glass_sphere(id: i32) -> Shape
    {
        let mut sphere = Self::new_sphere(id);
        let mut material = sphere.material;
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        sphere.material = material;
        sphere
    }

    pub fn new_plane(id: i32) -> Shape
    {
        let zero_point = create_point(0.0, 0.0, 0.0);
        let zero_vector = create_vector(0.0, 0.0, 0.0);

        Shape{id: id,
            transform: Matrix::identity(4),
            material: Material::new(),
            saved_ray: Ray::new(zero_point, zero_vector),
            specific: ShapeSpecific::Plane(Plane::new())}
    }

    pub fn new_cube(id: i32) -> Shape
    {
        let zero_point = create_point(0.0, 0.0, 0.0);
        let zero_vector = create_vector(0.0, 0.0, 0.0);

        Shape{id: id,
            transform: Matrix::identity(4),
            material: Material::new(),
            saved_ray: Ray::new(zero_point, zero_vector),
            specific: ShapeSpecific::Cube(Cube::new())}
    }

    pub fn test_shape(id: i32) -> Shape
    {
        Self::new_sphere(id)
    }

    pub fn get_transform(&self) -> Matrix
    {
        self.transform.clone()
    }

    pub fn set_transform(&mut self, transform: Matrix)
    {
        self.transform = transform;
    }

    pub fn get_material(&self) -> Material
    {
        self.material.clone()
    }

    pub fn set_material(&mut self, material: Material)
    {
        self.material = material;
    }

    pub fn intersect(&mut self, ray: Ray) -> Vec<f64>
    {
        let local_ray = ray.transform(self.transform.inverse());
        self.saved_ray = local_ray.clone();
        match self.specific.clone()
        {
            ShapeSpecific::Sphere(s) => s.local_intersect(local_ray),
            ShapeSpecific::Plane(p) => p.local_intersect(local_ray),
            ShapeSpecific::Cube(c) => c.local_intersect(local_ray),
        }
    }

    pub fn get_saved_ray(&self) -> Ray
    {
        self.saved_ray
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple
    {
        let inverse = self.get_transform().inverse();
        let local_point = inverse.clone().multiply_tuple(world_point);
        let local_normal = match self.specific.clone()
        {
            ShapeSpecific::Sphere(s) => s.local_normal_at(local_point),
            ShapeSpecific::Plane(p) => p.local_normal_at(local_point),
            ShapeSpecific::Cube(c) => c.local_normal_at(local_point),
        };
        let world_normal = inverse.transpose().multiply_tuple(local_normal);
        let v = world_normal.get_vec();
        let v2 = create_vector(v[0], v[1], v[2]); // resets world_normal.w to zero
        v2.normalize()
    }
}

impl PartialEq for Shape
{
    fn eq(&self, other: &Self) -> bool
    {
        match self.specific
        {
            ShapeSpecific::Sphere(_) =>
            {
                match other.specific
                {
                    ShapeSpecific::Sphere(_) => self.id == other.id,
                    _ => false,
                }
            },
            ShapeSpecific::Plane(_) =>
            {
                match other.specific
                {
                    ShapeSpecific::Plane(_) => self.id == other.id,
                    _ => false,
                }
            },
            ShapeSpecific::Cube(_) =>
            {
                match other.specific
                {
                    ShapeSpecific::Cube(_) => self.id == other.id,
                    _ => false,
                }
            },
        }
    }
}

impl fmt::Display for Shape
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.specific
        {
            ShapeSpecific::Sphere(_) => write!(f, "sphere {}", self.id),
            ShapeSpecific::Plane(_) => write!(f, "plane {}", self.id),
            ShapeSpecific::Cube(_) => write!(f, "cube {}", self.id),
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
        s4.set_material(m4.clone());
        assert_eq!(s4.get_material(), m4);

        // p.120 Scenario: Intersecting a scaled shape with a ray
        let r5 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let mut s5 = Shape::test_shape(5);
        s5.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let _xs5 = s5.intersect(r5);
        assert_eq!(s5.get_saved_ray().origin, create_point(0.0, 0.0, -2.5));
        assert_eq!(s5.get_saved_ray().direction, create_vector(0.0, 0.0, 0.5));

        // p.120 Scenario: Intersecting a translated shape with a ray
        let r6 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let mut s6 = Shape::test_shape(6);
        s6.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let _xs6 = s6.intersect(r6);
        assert_eq!(s6.get_saved_ray().origin, create_point(-5.0, 0.0, -5.0));
        assert_eq!(s6.get_saved_ray().direction, create_vector(0.0, 0.0, 1.0));
    }
}
