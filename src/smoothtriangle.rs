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
    pub e1: Tuple,
    pub e2: Tuple,
    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
}

impl SmoothTriangle
{
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple,
        n1: Tuple, n2: Tuple, n3: Tuple) -> Self
    {
        let e1 = p2.sub(p1);
        let e2 = p3.sub(p1);
        SmoothTriangle{p1: p1, p2: p2, p3: p3,
            e1: e1, e2: e2, n1: n1, n2: n2, n3: n3}
    }

    pub fn local_normal_at(&self, point: Tuple, hit_uv: (f64, f64)) -> Tuple
    {
        self.n2.multiply(hit_uv.0)
            .add(self.n3.multiply(hit_uv.1))
            .add(self.n1.multiply(1.0 - hit_uv.0 - hit_uv.1))
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<(f64, f64, f64)>
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
        let origin_cross_e1 = p1_to_origin.cross_product(self.e1);
        let v = f * ray.direction.dot_product(origin_cross_e1);
        if v < 0.0 || u + v > 1.0
        {
            return Vec::new();
        }
        let t = f * self.e2.dot_product(origin_cross_e1);
        vec![(t, u, v)]
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
    fn test_smoothtriangles_feature14()
    {
        // p.221 Scenario: Constructing a smooth triangle
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let n1 = create_vector(0.0, 1.0, 0.0);
        let n2 = create_vector(-1.0, 0.0, 0.0);
        let n3 = create_vector(1.0, 0.0, 0.0);
        let t14 = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);
        assert_eq!(t14.p1, p1);
        assert_eq!(t14.p2, p2);
        assert_eq!(t14.p3, p3);
        assert_eq!(t14.n1, n1);
        assert_eq!(t14.n2, n2);
        assert_eq!(t14.n3, n3);
    }

    fn create_tri() -> Shape
    {
        let p1 = create_point(0.0, 1.0, 0.0);
        let p2 = create_point(-1.0, 0.0, 0.0);
        let p3 = create_point(1.0, 0.0, 0.0);
        let n1 = create_vector(0.0, 1.0, 0.0);
        let n2 = create_vector(-1.0, 0.0, 0.0);
        let n3 = create_vector(1.0, 0.0, 0.0);
        Shape::new_smooth_triangle(15, p1, p2, p3, n1, n2, n3)
    }

    #[test]
    fn test_smoothtriangles_feature15()
    {
        // p.221 Scenario: An intersection can encapsulate u and v
        let t15 = create_tri();
        let i15 = Intersection::new_with_uv(3.5, t15, 0.2, 0.4);
        assert!(fuzzy_equal(i15.u, 0.2));
        assert!(fuzzy_equal(i15.v, 0.4));
    }

    #[test]
    fn test_smoothtriangles_feature16()
    {
        // p.221 Scenario: An intersection with a smooth triangle stores u/v
        let r16 = Ray::new(create_point(-0.2, 0.3, -2.0),
            create_vector(0.0, 0.0, 1.0));
        let mut t16 = create_tri();
        let i16 = t16.intersect(r16);
        assert_eq!(i16.len(), 1);
        assert!(fuzzy_equal(i16[0].1, 0.45));
        assert!(fuzzy_equal(i16[0].2, 0.25));
    }

    #[test]
    fn test_smoothtriangles_feature17()
    {
        // p.222 Scenario: A smooth triangle uses  u/v to interpolate the normal
        let mut t17 = create_tri();
        let i17 = Intersection::new_with_uv(1.0, t17.clone(), 0.45, 0.25);
        let n17 = t17.normal_at(create_point(0.0, 0.0, 0.0), (i17.u, i17.v));
        assert_eq!(n17, create_vector(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn test_smoothtriangles_feature18()
    {
        // p.223 Scenario: Preparing the normal on a smooth triangle
        let r18 = Ray::new(create_point(-0.2, 0.3, -2.0),
            create_vector(0.0, 0.0, 1.0));
        let mut t18 = create_tri();
        let i18 = Intersection::new_with_uv(1.0, t18.clone(), 0.45, 0.25);
        let comps18 = i18.prepare_computations(r18,
            Intersections::new(vec![i18.clone()]));
        assert_eq!(comps18.normalv, create_vector(-0.5547, 0.83205, 0.0));
    }
}
