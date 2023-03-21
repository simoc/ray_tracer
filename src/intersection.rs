use crate::sphere::*;

#[derive(Copy, Clone)]
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
}

pub struct Intersections
{
    intersections: Vec<Intersection>,
}

impl Intersections
{
    pub fn new(intersections: Vec<Intersection>) -> Self
    {
        Intersections{intersections: intersections}
    }

    pub fn count(&self) -> usize
    {
        self.intersections.len()
    }

    pub fn get_intersection(&self, index: usize) -> Intersection
    {
        self.intersections[index]
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
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);

        // p.64 Scenario: Aggregating intersections
        let s2 = Sphere::new(2);
        let i1 = Intersection::new(1.0, s2);
        let i2 = Intersection::new(2.0, s2);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get_intersection(0).object, s2);
        assert_eq!(xs.get_intersection(1).object, s2);
    }
}
