use std::fmt;
use std::cmp;
use std::rc::Rc;
use std::f64::consts::PI;
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

    pub fn local_intersect(&self, ray: Ray) -> Vec<(f64, f64, f64)>
    {
        let mut xs = Vec::<(f64, f64, f64)>::new();
        for shape in &self.child_shapes
        {
            let mut child_shape = shape.clone();
            let intersections = child_shape.intersect(ray);
            for tuv in intersections
            {
                xs.push(tuv);
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
        let mut separator = "";
        write!(f, "[")?;
        for child in &self.child_shapes
        {
            write!(f, "{}{}", separator, child)?;
            separator = ", ";
        }
        write!(f, "]")
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
        let xs4 = group4.local_intersect(r4);
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

    #[test]
    fn test_groups_feature6()
    {
        // p.197 Scenario: Intersecting a transformed group
        let mut group6 = Shape::new_group(6);
        group6.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut s6 = Shape::new_sphere(61);
        s6.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        group6.add_child(&mut s6);
        let r6 = Ray::new(create_point(10.0, 0.0, -10.0),
            create_vector(0.0, 0.0, 1.0));
        let xs6 = group6.intersect(r6);
        assert_eq!(xs6.len(), 2);
    }

    #[test]
    fn test_groups_feature7()
    {
        // p.198 Scenario: Converting a point from world to object space
        let mut group71 = Shape::new_group(71);
        let mut group72 = Shape::new_group(72);
        group71.set_transform(Matrix::rotation_y(PI / 2.0));
        group72.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        group71.add_child(&mut group72);
        let mut s73 = Shape::new_sphere(73);
        s73.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        group72.add_child(&mut s73);
        let p7 = s73.world_to_object(create_point(-2.0, 0.0, -10.0));
        assert_eq!(p7, create_point(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_groups_feature8()
    {
        // p.198 Scenario: Converting a normal from object to world space
        let mut group81 = Shape::new_group(81);
        let mut group82 = Shape::new_group(82);
        group81.set_transform(Matrix::rotation_y(PI / 2.0));
        group82.set_transform(Matrix::scaling(1.0, 2.0, 3.0));
        group81.add_child(&mut group82);
        let mut s83 = Shape::new_sphere(83);
        s83.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        group82.add_child(&mut s83);
        let sqrt3 = 3.0_f64.sqrt();
        let n8 = s83.normal_to_world(create_point(sqrt3 / 3.0,
            sqrt3 / 3.0, sqrt3 / 3.0));
        assert!(n8.approx_equal(create_vector(0.2857, 0.4286, -0.8571)));
    }

    #[test]
    fn test_groups_feature9()
    {
        // p.198 Scenario: Finding the normal on a child object
        let mut group91 = Shape::new_group(91);
        let mut group92 = Shape::new_group(92);
        group91.set_transform(Matrix::rotation_y(PI / 2.0));
        group92.set_transform(Matrix::scaling(1.0, 2.0, 3.0));
        group91.add_child(&mut group92);
        let mut s93 = Shape::new_sphere(93);
        s93.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        group92.add_child(&mut s93);
        let n9 = s93.normal_at(create_point(1.7321, 1.1547, -5.5774));
        assert!(n9.approx_equal(create_vector(0.2857, 0.4286, -0.8571)));
    }
}
