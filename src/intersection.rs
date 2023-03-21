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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_intersection_feature()
    {
        // p.63 Scenario: An intersection encapsulates t and object
        let s = Sphere::new(1);
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }
}
