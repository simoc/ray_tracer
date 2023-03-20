use std::fmt;
use crate::tuple::*;
use crate::ray::*;

#[derive(Copy, Clone, Debug)]
pub struct Sphere
{
    id: i32,
}

impl Sphere
{
    pub fn new(id: i32) -> Self
    {
        Sphere{id: id}
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64>
    {
        // the vector from the sphere's centre, to the ray origin
        // remember: the sphere is centred at the world origin
        let sphere_to_ray = ray.origin.sub(create_point(0.0, 0.0, 0.0));

        let a = ray.direction.dot_product(ray.direction);
        let b = 2.0 * sphere_to_ray.dot_product(ray.direction);
        let c = sphere_to_ray.dot_product(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0
        {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        return vec![t1, t2];
    }
}

impl PartialEq for Sphere
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}

impl fmt::Display for Sphere
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "sphere {}", self.id)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_spheres_feature()
    {
        let s1 = Sphere::new(1);
        let s2 = Sphere::new(2);
        assert_eq!(s1, s1);
        assert_ne!(s1, s2);

        // p.59 Scenario: A ray intersects a sphere at two points
        let r3 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let s3 = Sphere::new(3);
        let xs = s3.intersect(r3);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }
}
