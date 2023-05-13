use std::f64::consts::PI;
use crate::intersections::*;
use crate::matrix::*;
use crate::ray::*;
use crate::shape::*;
use crate::tuple::*;

// Manages groups and the shapes assigned to each group
// using integer group_id values to avoid dealing with object
// references.
#[derive(Clone, Debug)]
pub struct Groups
{
    child_shapes: Vec<Vec<Shape>>,
    child_groups: Vec<Vec<usize>>,
    transforms: Vec<Matrix>,
}

impl Groups
{
    pub fn new() -> Self
    {
        Groups{child_shapes: Vec::new(), child_groups: Vec::new(),
            transforms: Vec::new()}
    }

    pub fn create_group(&mut self) -> usize
    {
        let group_id = self.child_shapes.len();
        self.child_shapes.push(Vec::new());
        self.child_groups.push(Vec::new());
        self.transforms.push(Matrix::identity(4));
        group_id
    }

    pub fn add_child_shape(&mut self, group_id: usize, child: &mut Shape)
    {
        child.set_parent_id(group_id);
        self.child_shapes[group_id].push(child.clone());
    }

    pub fn get_child_shapes(&self, group_id: usize) -> Vec<Shape>
    {
        self.child_shapes[group_id].clone()
    }

    pub fn add_child_group(&mut self, group_id: usize, child_group_id: usize)
    {
        self.child_groups[group_id].push(child_group_id);
    }

    pub fn get_child_groups(&self, group_id: usize) -> Vec<usize>
    {
        self.child_groups[group_id].clone()
    }

    pub fn get_transform(&self, group_id: usize) -> Matrix
    {
        self.transforms[group_id].clone()
    }

    pub fn set_transform(&mut self, group_id: usize, transform: Matrix)
    {
        self.transforms[group_id] = transform;
    }

    pub fn local_intersect(&self, group_id: usize, ray: Ray) -> Intersections
    {
        let mut xs = Vec::new();
        let group_transform = self.get_transform(group_id);
        for shape in self.get_child_shapes(group_id)
        {
            let mut transformed_shape = shape.clone();
            let transform = transformed_shape.get_transform();
            transformed_shape.set_transform(group_transform.multiply(&transform));
            for t in transformed_shape.intersect(ray)
            {
                xs.push(Intersection::new(t, shape.clone()));
            }
        }
        return Intersections::new(xs);
    }

    pub fn world_to_object(&self, shape: Shape, world_point: Tuple) -> Tuple
    {
        // TODO implement from p.198 pseudocode
        create_point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_groups_feature1()
    {
        // p.195 Scenario: Creating a new group
        let mut groups1 = Groups::new();
        let group_id1 = groups1.create_group();
        assert_eq!(groups1.get_transform(group_id1), Matrix::identity(4));
        assert!(groups1.get_child_shapes(group_id1).is_empty());
        assert!(groups1.get_child_groups(group_id1).is_empty());
    }

    #[test]
    fn test_groups_feature2()
    {
        // p.195 Scenario: A shape has a parent attribute
        let s2 = Shape::test_shape(2);
        assert!(s2.get_parent_id().is_none());
    }

    #[test]
    fn test_groups_feature3()
    {
        // p.195 Scenario: Adding a child to a group
        let mut groups3 = Groups::new();
        let group_id3 = groups3.create_group();
        let mut s3 = Shape::test_shape(3);
        groups3.add_child_shape(group_id3, &mut s3);
        assert!(groups3.get_child_shapes(group_id3).contains(&s3));
        assert_eq!(s3.get_parent_id().unwrap(), group_id3);
    }

    #[test]
    fn test_groups_feature4()
    {
        // p.195 Scenario: Intersecting a ray with an empty group
        let mut groups4 = Groups::new();
        let group_id4 = groups4.create_group();
        let r4 = Ray::new(create_point(0.0, 0.0, 0.0),
            create_vector(0.0, 0.0, 1.0));
        let xs4 = groups4.local_intersect(group_id4, r4);
        assert_eq!(xs4.count(), 0);
    }

    #[test]
    fn test_groups_feature5()
    {
        // p.196 Scenario: Intersecting a ray with an nonempty group
        let mut groups5 = Groups::new();
        let group_id5 = groups5.create_group();
        let mut s51 = Shape::test_shape(51);
        let mut s52 = Shape::test_shape(52);
        s52.set_transform(Matrix::translation(0.0, 0.0, -3.0));
        let mut s53 = Shape::test_shape(53);
        s53.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        groups5.add_child_shape(group_id5, &mut s51);
        groups5.add_child_shape(group_id5, &mut s52);
        groups5.add_child_shape(group_id5, &mut s53);
        let r5 = Ray::new(create_point(0.0, 0.0, -5.0),
            create_vector(0.0, 0.0, 1.0));
        let xs5 = groups5.local_intersect(group_id5, r5);
        assert_eq!(xs5.count(), 4);
        assert_eq!(xs5.get_intersection(0).object, s52);
        assert_eq!(xs5.get_intersection(1).object, s52);
        assert_eq!(xs5.get_intersection(2).object, s51);
        assert_eq!(xs5.get_intersection(3).object, s51);
    }

    #[test]
    fn test_groups_feature6()
    {
        // p.197 Scenario: Intersecting a transformed group
        let mut groups6 = Groups::new();
        let group_id6 = groups6.create_group();
        groups6.set_transform(group_id6, Matrix::scaling(2.0, 2.0, 2.0));
        let mut s6 = Shape::new_sphere(6);
        s6.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        groups6.add_child_shape(group_id6, &mut s6);
        let r6 = Ray::new(create_point(10.0, 0.0, -10.0),
            create_vector(0.0, 0.0, 1.0));
        let xs6 = groups6.local_intersect(group_id6, r6);
        assert_eq!(xs6.count(), 2);
    }

    #[test]
    fn test_groups_feature7()
    {
        // p.198 Scenario: Converting a point from world to object space
        let mut groups7 = Groups::new();
        let group_id71 = groups7.create_group();
        groups7.set_transform(group_id71, Matrix::rotation_y(PI / 2.0));
        let group_id72 = groups7.create_group();
        groups7.set_transform(group_id72, Matrix::scaling(2.0, 2.0, 2.0));
        groups7.add_child_group(group_id71, group_id72);
        let mut s7 = Shape::new_sphere(7);
        s7.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        groups7.add_child_shape(group_id72, &mut s7);
        let p7 = groups7.world_to_object(s7, create_point(-2.0, 0.0, -10.0));
    }
}
