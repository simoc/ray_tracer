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

    pub fn local_intersect(&self, ray: Ray, group_transform: Matrix) -> Vec<f64>
    {
        let mut xs = Vec::<f64>::new();
        for shape in &self.child_shapes
        {
            let mut transformed_shape = shape.clone();
            let transform = transformed_shape.get_transform();
            transformed_shape.set_transform(group_transform.multiply(&transform));
            let intersections = transformed_shape.intersect(ray);
            for t in intersections
            {
                xs.push(t);
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        return xs;
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
        assert_eq!(*s3.get_parent().unwrap(), group3);
        assert!(s4.get_parent().is_none());
    }

    #[test]
    fn test_groups_feature4()
    {
        // p.195 Scenario: Intersecting a ray with an empty group
        let mut group4 = Group::new();
        let r4 = Ray::new(create_point(0.0, 0.0, 0.0),
            create_vector(0.0, 0.0, 1.0));
        let xs4 = group4.local_intersect(r4, Matrix::identity(4));
        assert_eq!(xs4.len(), 0);
    }

    #[test]
    fn test_groups_feature5()
    {
        // p.196 Scenario: Intersecting a ray with an nonempty group
        let mut group5 = Shape::new_group(5);
        let mut s51 = Shape::test_shape(51);
        let mut s52 = Shape::test_shape(52);
        s52.set_transform(Matrix::translation(0.0, 0.0, -3.0));
        let mut s53 = Shape::test_shape(53);
        s53.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        group5.add_child(&mut s51);
        group5.add_child(&mut s52);
        group5.add_child(&mut s53);
        let r5 = Ray::new(create_point(0.0, 0.0, -5.0),
            create_vector(0.0, 0.0, 1.0));
        let xs5 = group5.intersect(r5);
        assert_eq!(xs5.len(), 4);
    }
}
