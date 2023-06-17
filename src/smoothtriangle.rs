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
pub struct SmoothTriangle
{
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
}

impl SmoothTriangle
{
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple,
        n1: Tuple, n2: Tuple, n3: Tuple) -> Self
    {
        SmoothTriangle{p1: p1, p2: p2, p3: p3, n1: n1, n2: n2, n3: n3}
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple
    {
        create_vector(0.0, 0.0, 0.0)
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64>
    {
        vec![]
    }
}

impl fmt::Display for SmoothTriangle
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "smoothtriangle")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_smoothtriangles_feature1()
    {
        // p.221 Scenario: Constructing a smooth triangle
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let n1 = create_vector(0.0, 1.0, 0.0);
        let n2 = create_vector(-1.0, 0.0, 0.0);
        let n3 = create_vector(1.0, 0.0, 0.0);
        let t1 = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);
        assert_eq!(t1.p1, p1);
        assert_eq!(t1.p2, p2);
        assert_eq!(t1.p3, p3);
        assert_eq!(t1.n1, n1);
        assert_eq!(t1.n2, n2);
        assert_eq!(t1.n3, n3);
    }
}
