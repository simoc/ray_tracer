use crate::material::*;
use crate::matrix::*;
use crate::sphere::*;

pub trait Shape
{
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material);
}

pub fn test_shape(id: i32) -> Sphere
{
    Sphere::new(id)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_shape_feature()
    {
        // p.119 Scenario: The default transformation
        let s1 = test_shape(1);
        assert_eq!(s1.get_transform(), Matrix::identity(4));

        // p.119 Scenario: Assigning a transformation
        let mut s2 = test_shape(2);
        s2.set_transform(Matrix::translation(2.0, 3.0, 4.0));
        assert_eq!(s2.get_transform(), Matrix::translation(2.0, 3.0, 4.0));

        // p.119 Scenario: The default material
        let s3 = test_shape(3);
        assert_eq!(s1.get_material(), Material::new());

        // p.119 Scenario: Assigning a material
        let mut s4 = test_shape(4);
        let mut m4 = Material::new();
        m4.ambient = 1.0;
        s4.set_material(m4);
        assert_eq!(s4.get_material(), m4);
    }
}
