use std::fmt;
use std::f64::consts::PI;
use crate::arithmetic::*;
use crate::tuple::*;
use crate::ray::*;
use crate::matrix::*;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct Plane
{
    id: i32,
    transform: Matrix,
    material: Material,
    saved_ray: Ray,
}

impl Plane
{
    pub fn get_local_transform(&self) -> Matrix
    {
        self.transform.clone()
    }

    pub fn set_local_transform(&mut self, transform: Matrix)
    {
        self.transform = transform;
    }

    pub fn get_local_material(&self) -> Material
    {
        self.material
    }

    pub fn set_local_material(&mut self, material: Material)
    {
        self.material = material;
    }

    pub fn get_id(&self) -> i32
    {
        self.id
    }
}

impl Plane
{
    pub fn new(id: i32) -> Self
    {
        let zero_point = create_point(0.0, 0.0, 0.0);
        let zero_vector = create_vector(0.0, 0.0, 0.0);
        Plane{id: id, transform: Matrix::identity(4),
            material: Material::new(),
            saved_ray: Ray::new(zero_point, zero_vector)}
    }

    pub fn local_intersect(&mut self, ray: Ray) -> Vec<f64>
    {
        if ray.direction.get_vec()[2] < EPSILON
        {
            // empty set -- no intersection
            return Vec::new();
        }
        // TODO: remaining intersection logic goes here
        return Vec::new();
    }

    pub fn local_get_saved_ray(&self) -> Ray
    {
        self.saved_ray
    }

    pub fn local_set_saved_ray(&mut self, saved_ray: Ray)
    {
        self.saved_ray = saved_ray;
    }

    pub fn local_normal_at(&self, _local_point: Tuple) -> Tuple
    {
        create_vector(0.0, 1.0, 0.0)
    }
}

impl PartialEq for Plane
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}

impl fmt::Display for Plane
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "plane {}", self.id)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_planes_feature()
    {
        // p.122 Scenario: The normal of a plane is constant everywhere
        let p1 = Plane::new(1);
        let n11 = p1.local_normal_at(create_point(0.0, 0.0, 0.0));
        let n12 = p1.local_normal_at(create_point(10.0, 0.0, -10.0));
        let n13 = p1.local_normal_at(create_point(-5.0, 0.0, 150.0));
        assert_eq!(n11, create_vector(0.0, 1.0, 0.0));
        assert_eq!(n12, create_vector(0.0, 1.0, 0.0));
        assert_eq!(n13, create_vector(0.0, 1.0, 0.0));

        // p.123 Scenario: Intersect with a ray parallel to the plane
        let mut p2 = Plane::new(2);
        let r2 = Ray::new(create_point(0.0, 10.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let xs2 = p2.local_intersect(r2);
        assert_eq!(xs2.len(), 0);

        // p.123 Scenario: Intersect with a coplanar ray
        let mut p3 = Plane::new(3);
        let r3 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let xs3 = p3.local_intersect(r3);
        assert_eq!(xs3.len(), 0);
    }
}
