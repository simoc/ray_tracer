use std::fmt;
use std::cmp;
use std::rc::Rc;
use crate::arithmetic::*;
use crate::intersections::*;
use crate::matrix::*;
use crate::tuple::*;
use crate::ray::*;
use crate::ray::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct Group
{
    pub child_shapes: Vec<Shape>,
}

// A collection of other Shapes
impl Group
{
    pub fn new() -> Self
    {
        Group{child_shapes: Vec::new()}
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64>
    {
        Vec::new()
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple
    {
        create_vector(0.0, 0.0, 1.0)
    }
}

impl fmt::Display for Group
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "group")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_group_feature1()
    {
        // p.195 Scenario: Creating a new group
        let group1 = Shape::new_group(1);
        assert_eq!(group1.get_transform(), Matrix::identity(4));
        assert!(group1.get_children().is_empty());
    }

    #[test]
    fn test_group_feature2()
    {
        // p.195 Scenario: A shape has a parent attribute
        let s2 = Shape::test_shape(2);
        assert!(s2.get_parent().is_none());
    }

    #[test]
    fn test_group_feature3()
    {
        // p.195 Scenario: Adding a child to a group
        let mut group3 = Shape::new_group(3);
        let mut s3 = Shape::test_shape(3);
        group3.add_child(&mut s3);
        let mut s4 = Shape::test_shape(4);
        assert!(group3.get_children().contains(&s3));
        assert!(!group3.get_children().contains(&s4));
        assert!(!s3.get_parent().is_none());
        assert!(s4.get_parent().is_none());
    }

}
