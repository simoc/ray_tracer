use crate::tuple::*;
use crate::arithmetic::*;

#[derive(Copy, Clone, Debug)]
pub struct Material
{
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material
{
    pub fn new() -> Self
    {
        Material{color: create_color(1.0, 1.0, 1.0), ambient: 0.1, diffuse: 0.9,
            specular: 0.9, shininess: 200.0}
    }
}

impl PartialEq for Material
{
    fn eq(&self, other: &Self) -> bool
    {
        self.color == other.color &&
            fuzzy_equal(self.ambient, other.ambient) &&
            fuzzy_equal(self.diffuse, other.diffuse) &&
            fuzzy_equal(self.specular, other.specular) &&
            fuzzy_equal(self.shininess, other.shininess)
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_material_feature()
    {
        // p.85 Scenario: The default material
        let material1 = Material::new();
        assert_eq!(material1.color, create_color(1.0, 1.0, 1.0));
        assert_eq!(material1.ambient, 0.1);
        assert_eq!(material1.diffuse, 0.9);
        assert_eq!(material1.specular, 0.9);
        assert_eq!(material1.shininess, 200.0);
    }
}
