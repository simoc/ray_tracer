use std::fmt;
use crate::tuple::*;
use crate::ray::*;
use crate::matrix::*;

#[derive(Clone, Debug)]
pub struct Sphere
{
    id: i32,
    transform: Matrix,
}

impl Sphere
{
    pub fn new(id: i32) -> Self
    {
        Sphere{id: id, transform: Matrix::identity(4)}
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64>
    {
        let ray2 = ray.transform(self.transform.inverse());

        // the vector from the sphere's centre, to the ray origin
        // remember: the sphere is centred at the world origin
        let sphere_to_ray = ray2.origin.sub(create_point(0.0, 0.0, 0.0));

        let a = ray2.direction.dot_product(ray2.direction);
        let b = 2.0 * sphere_to_ray.dot_product(ray2.direction);
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

    pub fn get_transform(&self) -> Matrix
    {
        self.transform.clone()
    }

    pub fn set_transform(&mut self, transform: Matrix)
    {
        self.transform = transform;
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple
    {
        let v = p.sub(create_point(0.0, 0.0, 0.0));
        v.normalize()
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
        let xs3 = s3.intersect(r3);
        assert_eq!(xs3.len(), 2);
        assert_eq!(xs3[0], 4.0);
        assert_eq!(xs3[1], 6.0);

        // p.60 Scenario: A ray intersects a sphere at a tangent
        let r4 = Ray::new(create_point(0.0, 1.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let s4 = Sphere::new(4);
        let xs4 = s4.intersect(r4);
        assert_eq!(xs4.len(), 2);
        assert_eq!(xs4[0], 5.0);
        assert_eq!(xs4[1], 5.0);

        // p.60 Scenario: A ray misses a sphere
        let r5 = Ray::new(create_point(0.0, 2.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let s5 = Sphere::new(5);
        let xs5 = s5.intersect(r5);
        assert_eq!(xs5.len(), 0);

        // p.61 Scenario: A ray originates inside a sphere
        let r6 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let s6 = Sphere::new(6);
        let xs6 = s6.intersect(r6);
        assert_eq!(xs6.len(), 2);
        assert_eq!(xs6[0], -1.0);
        assert_eq!(xs6[1], 1.0);

        // p.62 Scenario: A sphere is behind a ray
        let r7 = Ray::new(create_point(0.0, 0.0, 5.0), create_vector(0.0, 0.0, 1.0));
        let s7 = Sphere::new(7);
        let xs7 = s7.intersect(r7);
        assert_eq!(xs7.len(), 2);
        assert_eq!(xs7[0], -6.0);
        assert_eq!(xs7[1], -4.0);

        // p.69 Scenario: A sphere's default transformation
        let s8 = Sphere::new(8);
        assert_eq!(s8.get_transform(), Matrix::identity(4));

        // p.69 Scenario: Changing a sphere's transformation
        let mut s9 = Sphere::new(9);
        let t9 = Matrix::translation(2.0, 3.0, 4.0);
        s9.set_transform(t9.clone());
        assert_eq!(s9.get_transform(), t9);

        // p.70 Scenario: Intersecting a scaled sphere with a ray
        let r10 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let mut s10 = Sphere::new(10);
        s10.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs10 = s10.intersect(r10);
        assert_eq!(xs10.len(), 2);
        assert_eq!(xs10[0], 3.0);
        assert_eq!(xs10[1], 7.0);

        // p.70 Scenario: Intersecting a translated sphere with a ray
        let r11 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let mut s11 = Sphere::new(11);
        s11.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs11 = s11.intersect(r11);
        assert_eq!(xs11.len(), 0);
    }

    #[test]
    fn test_spheres_feature_shading()
    {
        // p.78 Scenario: The normal on a sphere at a point on the x axis
        let s1 = Sphere::new(1);
        let n1 = s1.normal_at(create_point(1.0, 0.0, 0.0));
        assert_eq!(n1, create_vector(1.0, 0.0, 0.0));
    }
}
