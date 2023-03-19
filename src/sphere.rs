use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Sphere
{
    id: i32,
}

impl Sphere
{
    pub fn new(id: i32) -> Self
    {
        Sphere{id: id}
	}
}

impl PartialEq for Sphere
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
	}
}

impl fmt::Display for Sphere
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "sphere {}", self.id)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_spheres_feature()
    {
        let s1 = Sphere::new(1);
        let s2 = Sphere::new(2);
        assert_eq!(s1, s1);
        assert_ne!(s1, s2);
    }
}
