use std::fmt;
use crate::computations::*;
use crate::ray::*;
use crate::sphere::*;

#[derive(Clone, Debug)]
pub struct Intersection
{
    pub t: f64,
    pub object: Sphere,
}

impl Intersection
{
    pub fn new(t: f64, object: Sphere) -> Self
    {
        Intersection{t: t, object: object}
    }

    pub fn prepare_computation(&self, ray: Ray) -> Computations
    {
        // precompute some useful values
        let point = ray.position(self.t);
        Computations::new(self.t, self.object.clone(), point,
            ray.direction.negate(),
            self.object.normal_at(point))
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
        let s = Sphere::new(1);
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);

        // p.64 Scenario: Aggregating intersections
        let s2 = Sphere::new(2);
        let i1 = Intersection::new(1.0, s2.clone());
        let i2 = Intersection::new(2.0, s2.clone());
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get_intersection(0).object, s2.clone());
        assert_eq!(xs.get_intersection(1).object, s2.clone());

        // p.65 Scenario: The hit, when all intersections have positive t
        let s3 = Sphere::new(3);
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
        let s4 = Sphere::new(4);
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
        let s5 = Sphere::new(5);
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
        let s6 = Sphere::new(6);
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
}
