use crate::tuple::*;

#[derive(Copy, Clone, Debug)]
pub struct PointLight
{
    pub position: Tuple,
    pub intensity: Tuple
}

impl PointLight
{
    pub fn new(position: Tuple, intensity: Tuple) -> Self
    {
        PointLight{position: position, intensity: intensity}
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_light_feature()
    {
        // p.84 Scenario: A point light has a position and an intensity
        let intensity1 = create_color(1.0, 1.0, 1.0);
        let position1 = create_point(0.0, 0.0, 0.0);
        let light1 = PointLight::new(position1, intensity1);
        assert_eq!(light1.position, position1);
        assert_eq!(light1.intensity, intensity1);
    }
}
