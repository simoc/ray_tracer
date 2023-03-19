use std::fmt;
use crate::tuple::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray
{
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray
{
    pub fn new(origin: Tuple, direction: Tuple) -> Self
    {
        Ray{origin: origin, direction: direction}
    }

    pub fn position(&self, t: f64) -> Tuple
    {
        let ov = self.origin.get_vec();
        let dv = self.direction.get_vec();
        create_point(ov[0] + t * dv[0],
            ov[1] + t * dv[1],
            ov[2] + t * dv[2])
    }
}

impl fmt::Display for Ray
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "origin {} direction {}", self.origin, self.direction)
    }
}

impl PartialEq for Ray
{
    fn eq(&self, other: &Self) -> bool
    {
        self.origin == other.origin && self.direction == other.direction
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_rays_feature()
    {
        // p.58 Scenario: Creating and querying a ray
        let origin = create_point(1.0, 2.0, 3.0);
        let direction = create_vector(4.0, 5.0, 6.0);
        let r1 = Ray::new(origin, direction);
        assert_eq!(r1.origin, origin);
        assert_eq!(r1.direction, direction);

        // p.58 Scenario: Computing a point from a distance
        let r2 = Ray::new(create_point(2.0, 3.0, 4.0), create_vector(1.0, 0.0, 0.0));
        assert_eq!(r2.position(0.0), create_point(2.0, 3.0, 4.0));
        assert_eq!(r2.position(1.0), create_point(3.0, 3.0, 4.0));
        assert_eq!(r2.position(-1.0), create_point(1.0, 3.0, 4.0));
        assert_eq!(r2.position(2.5), create_point(4.5, 3.0, 4.0));
    }
}
