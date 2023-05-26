use std::fmt;
use std::cmp;
use crate::arithmetic::*;
use crate::intersections::*;
use crate::tuple::*;
use crate::ray::*;
use crate::matrix::*;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct Triangle
{
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
}

impl Triangle
{
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self
    {
        let e1 = p2.sub(p1);
        let e2 = p3.sub(p1);
        let normal = e2.cross_product(e1).normalize();
        Triangle{p1: p1, p2: p2, p3: p3, e1: e1, e2: e2, normal: normal}
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple
    {
        self.normal
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64>
    {
        let dir_cross_e2 = ray.direction.cross_product(self.e2);
        let det = self.e1.dot_product(dir_cross_e2);
        if det.abs() < EPSILON
        {
            return Vec::new();
        }
        let f = 1.0 / det;
        let p1_to_origin = ray.origin.sub(self.p1);
        let u = f * p1_to_origin.dot_product(dir_cross_e2);
        if u < 0.0 || u > 1.0
        {
            return Vec::new();
        }
        // A bogus intersection to ensure the result isn't a false positive
        vec![1.0]
    }
}

impl fmt::Display for Triangle
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "triangle")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_triangles_feature1()
    {
        // p.208 Scenario: Constructing a triangle
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let t1 = Triangle::new(p1, p2, p3);
        assert_eq!(t1.p1, p1);
        assert_eq!(t1.p2, p2);
        assert_eq!(t1.p3, p3);
        assert_eq!(t1.e1, create_vector(-1.0, -1.0, 0.0));
        assert_eq!(t1.e2, create_vector(1.0, -1.0, 0.0));
        assert_eq!(t1.normal, create_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_triangles_feature2()
    {
        // p.209 Scenario: Finding the normal on a triangle
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let t1 = Triangle::new(p1, p2, p3);
        let n1 = t1.local_normal_at(create_point(0.0, 0.5, 0.0));
        let n2 = t1.local_normal_at(create_point(-0.5, 0.75, 0.0));
        let n3 = t1.local_normal_at(create_point(0.5, 0.25, 0.0));
        assert_eq!(n1, t1.normal);
        assert_eq!(n2, t1.normal);
        assert_eq!(n3, t1.normal);
    }

    #[test]
    fn test_triangles_feature3()
    {
        // p.210 Scenario: Intersecting a ray parallel to the triangle
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let t3 = Triangle::new(p1, p2, p3);
        let r3 = Ray::new(create_point(0.0, -1.0, -2.0), create_vector(0.0, 1.0, 0.0));
        let xs3 = t3.local_intersect(r3);
        assert!(xs3.is_empty());
    }

    #[test]
    fn test_triangles_feature4()
    {
        // p.210 Scenario: A ray misses the p1-p3 edge
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let t4 = Triangle::new(p1, p2, p3);
        let r4 = Ray::new(create_point(1.0, 1.0, -2.0), create_vector(0.0, 0.0, 1.0));
        let xs4 = t4.local_intersect(r4);
        assert!(xs4.is_empty());
    }
}
