use std::fmt;
use std::cmp;
use std::f64::consts::PI;
use crate::arithmetic::*;
use crate::intersections::*;
use crate::tuple::*;
use crate::ray::*;
use crate::matrix::*;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct Cylinder
{
}

impl Cylinder
{
    pub fn new() -> Self
    {
        Cylinder{}
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64>
    {
        let vd = ray.direction.get_vec();
        let a = (vd[0] * vd[0]) + (vd[2] * vd[2]);

        // ray is parallel to the y axis
        if fuzzy_equal(a, 0.0)
        {
            return vec![];
        }

        let vo = ray.origin.get_vec();
        let b = 2.0 * vo[0] * vd[0] + 2.0 * vo[2] * vd[2];
        let c = (vo[0] * vo[0]) + (vo[2] * vo[2]) - 1.0;
        let disc = (b * b) - 4.0 * a * c;

        // ray does not intersect the cylinder
        if disc < 0.0
        {
            return vec![];
        }

        // this is just a placeholder, to ensure the tests
        // passs that expect the ray to miss.
        return vec![1.0];
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple
    {
        return create_vector(0.0, 0.0, 0.0);
    }
}

impl fmt::Display for Cylinder
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "cylinder")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_cylinders_feature1()
    {
        // p.178 Scenario: A ray misses a cylinder
        let c1 = Cylinder::new();
        let origins1 = vec![create_point(1.0, 0.0, 0.0),
            create_point(0.0, 0.0, 0.0),
            create_point(0.0, 0.0, -5.0)];
        let directions1 = vec![create_vector(0.0, 1.0, 0.0),
            create_vector(0.0, 1.0, 0.0),
            create_vector(1.0, 1.0, 1.0)];

        for i in 0..origins1.len()
        {
            let r1 = Ray::new(origins1[i], directions1[i]);
            let xs1 = c1.local_intersect(r1);
            assert_eq!(xs1.len(), 0);
        }
    }
}
