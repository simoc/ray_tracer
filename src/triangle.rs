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
}
