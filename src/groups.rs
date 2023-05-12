use crate::matrix::*;
use crate::shape::*;

// Manages groups and the shapes assigned to each group
// using integer group_id values to avoid dealing with object
// references.
#[derive(Clone, Debug)]
pub struct Groups
{
    group_children: Vec<Vec<Shape>>,
    group_transforms: Vec<Matrix>,
}

impl Groups
{
    pub fn new() -> Self
    {
        Groups{group_children: Vec::new(), group_transforms: Vec::new()}
    }

    pub fn create_group(&mut self) -> usize
    {
        let group_id = self.group_children.len();
        self.group_children.push(Vec::new());
        self.group_transforms.push(Matrix::identity(4));
        group_id
    }

    pub fn add_child(&mut self, group_id: usize, child: &mut Shape)
    {
        child.set_parent_id(group_id);
        self.group_children[group_id].push(child.clone());
    }

    pub fn get_children(&self, group_id: usize) -> Vec<Shape>
    {
        self.group_children[group_id].clone()
    }

    pub fn get_transform(&self, group_id: usize) -> Matrix
    {
        self.group_transforms[group_id].clone()
    }

    pub fn set_transform(&mut self, group_id: usize, transform: Matrix)
    {
        self.group_transforms[group_id] = transform;
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
        assert!(groups1.get_children(group_id1).is_empty());
    }

    #[test]
    fn test_groups_feature2()
    {
        // p.195 Scenario: A shape has a parent attribute
        let mut groups2 = Groups::new();
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
        groups3.add_child(group_id3, &mut s3);
        assert!(groups3.get_children(group_id3).contains(&s3));
        assert_eq!(s3.get_parent_id().unwrap(), group_id3);
    }
}
