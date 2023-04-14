use std::fmt;
use crate::arithmetic::*;
use crate::computations::*;
use crate::matrix::*;
use crate::ray::*;
use crate::shape::*;
use crate::sphere::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct Intersection
{
    pub t: f64,
    pub object: Shape,
}

impl Intersection
{
    pub fn new(t: f64, object: Shape) -> Self
    {
        Intersection{t: t, object: object}
    }

    pub fn prepare_computation(&self, ray: Ray) -> Computations
    {
        // precompute some useful values
        let point = ray.position(self.t);
        let eyev = ray.direction.negate();
        let mut normalv = self.object.normal_at(point);
        let inside: bool;
        if normalv.dot_product(eyev) < 0.0
        {
            inside = true;
            normalv = normalv.negate();
        }
        else
        {
            inside = false;
        }
        let over_point = point.add(normalv.multiply(EPSILON));
        Computations::new(self.t, self.object.clone(), point,
            eyev, normalv, inside, over_point)
    }
}

impl PartialEq for Intersection
{
    fn eq(&self, other: &Self) -> bool
    {
        self.t == other.t && self.object == other.object
    }
}


impl fmt::Display for Intersection
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "t: {} object: {}", self.t, self.object)
    }
}

pub struct Intersections
{
    intersections: Vec<Intersection>,
}

impl Intersections
{
    pub fn new(intersections: Vec<Intersection>) -> Self
    {
        let mut unsorted = intersections;
        unsorted.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections{intersections: unsorted}
    }

    pub fn count(&self) -> usize
    {
        self.intersections.len()
    }

    pub fn get_intersection(&self, index: usize) -> Intersection
    {
        self.intersections[index].clone()
    }

    pub fn hit(&self) -> Option<Intersection>
    {
        for i in 0..self.intersections.len()
        {
            // Return intersection with lowest non-negative t value.
            if self.intersections[i].t >= 0.0
            {
                return Some(self.intersections[i].clone());
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_intersections_feature()
    {
        // p.63 Scenario: An intersection encapsulates t and object
        let s = Shape::new_sphere(1);
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);

        // p.64 Scenario: Aggregating intersections
        let s2 = Shape::new_sphere(2);
        let i1 = Intersection::new(1.0, s2.clone());
        let i2 = Intersection::new(2.0, s2.clone());
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get_intersection(0).object, s2.clone());
        assert_eq!(xs.get_intersection(1).object, s2.clone());

        // p.65 Scenario: The hit, when all intersections have positive t
        let s3 = Shape::new_sphere(3);
        let j31 = Intersection::new(1.0, s3.clone());
        let j32 = Intersection::new(2.0, s3.clone());
        let xs3 = Intersections::new(vec![j31.clone(), j32.clone()]);
        let j3 = xs3.hit();
        match j3
        {
            Some(x) => assert_eq!(x, j31),
            None => assert!(false),
        }

        // p.65 Scenario: The hit, when some intersections have negative t
        let s4 = Shape::new_sphere(4);
        let j41 = Intersection::new(-1.0, s4.clone());
        let j42 = Intersection::new(1.0, s4.clone());
        let xs4 = Intersections::new(vec![j41.clone(), j42.clone()]);
        let j4 = xs4.hit();
        match j4
        {
            Some(x) => assert_eq!(x, j42),
            None => assert!(false),
        }

        // p.65 Scenario: The hit, when some intersections have negative t
        let s5 = Shape::new_sphere(5);
        let j51 = Intersection::new(-2.0, s5.clone());
        let j52 = Intersection::new(-1.0, s5.clone());
        let xs5 = Intersections::new(vec![j51.clone(), j52.clone()]);
        let j5 = xs5.hit();
        match j5
        {
            Some(_) => assert!(false),
            None => assert!(true),
        }

        // p.66 Scenario: The hit is always the lowest non-negative intersection
        let s6 = Shape::new_sphere(6);
        let j61 = Intersection::new(5.0, s6.clone());
        let j62 = Intersection::new(7.0, s6.clone());
        let j63 = Intersection::new(-3.0, s6.clone());
        let j64 = Intersection::new(2.0, s6.clone());
        let xs6 = Intersections::new(vec![j61.clone(), j62.clone(), j63.clone(), j64.clone()]);
        let j6 = xs6.hit();
        match j6
        {
            Some(x) => assert_eq!(x, j64),
            None => assert!(false),
        }
    }

    #[test]
    fn test_intersections_shadow_feature()
    {
        // p.115 Scenario: The hit should offset the point
        let r1 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let mut shape1 = Shape::new_sphere(1);
        shape1.set_transform(Matrix::translation(0.0, 0.0, 1.0));
        let i1 = Intersection::new(5.0, shape1);
        let comps1 = i1.prepare_computation(r1);
        assert!(comps1.over_point.get_vec()[2] < -EPSILON / 2.0);
        assert!(comps1.point.get_vec()[2] > comps1.over_point.get_vec()[2]);
    }
}
