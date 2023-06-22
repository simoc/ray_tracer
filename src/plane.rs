use std::fmt;
use std::f64::consts::PI;
use crate::arithmetic::*;
use crate::intersections::*;
use crate::tuple::*;
use crate::ray::*;
use crate::matrix::*;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;

// A plane in x and z axes, passing through the origin
#[derive(Clone, Debug)]
pub struct Plane
{
}

impl Plane
{
    pub fn new() -> Self
    {
        Plane{}
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<(f64, f64, f64)>
    {
        let u = 0.0;
        let v = 0.0;
        if ray.direction.get_vec()[1].abs() < EPSILON
        {
            // empty set -- no intersection
            return Vec::new();
        }
        let t = -ray.origin.get_vec()[1] / ray.direction.get_vec()[1];
        return vec![(t, u, v)];
    }

    pub fn local_normal_at(&self, _local_point: Tuple) -> Tuple
    {
        create_vector(0.0, 1.0, 0.0)
    }
}

impl fmt::Display for Plane
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "plane")
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
        let p1 = Plane::new();
        let n11 = p1.local_normal_at(create_point(0.0, 0.0, 0.0));
        let n12 = p1.local_normal_at(create_point(10.0, 0.0, -10.0));
        let n13 = p1.local_normal_at(create_point(-5.0, 0.0, 150.0));
        assert_eq!(n11, create_vector(0.0, 1.0, 0.0));
        assert_eq!(n12, create_vector(0.0, 1.0, 0.0));
        assert_eq!(n13, create_vector(0.0, 1.0, 0.0));

        // p.123 Scenario: Intersect with a ray parallel to the plane
        let p2 = Plane::new();
        let r2 = Ray::new(create_point(0.0, 10.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let xs2 = p2.local_intersect(r2);
        assert_eq!(xs2.len(), 0);

        // p.123 Scenario: Intersect with a coplanar ray
        let p3 = Plane::new();
        let r3 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let xs3 = p3.local_intersect(r3);
        assert_eq!(xs3.len(), 0);

        // p.123 Scenario: A ray intersecting a plane from above
        let p4 = Plane::new();
        let r4 = Ray::new(create_point(0.0, 1.0, 0.0), create_vector(0.0, -1.0, 0.0));
        let xs4 = p4.local_intersect(r4);
        assert_eq!(xs4.len(), 1);
        assert!(fuzzy_equal(xs4[0].0, 1.0));

        // p.123 Scenario: A ray intersecting a plane from below
        let p5 = Plane::new();
        let r5 = Ray::new(create_point(0.0, -1.0, 0.0), create_vector(0.0, 1.0, 0.0));
        let xs5 = p5.local_intersect(r5);
        assert_eq!(xs5.len(), 1);
        assert!(fuzzy_equal(xs5[0].0, 1.0));
    }
}
