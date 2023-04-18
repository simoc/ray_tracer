use crate::tuple::*;
use crate::arithmetic::*;
use crate::pointlight::*;
use crate::pattern::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct Material
{
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
}

impl Material
{
    pub fn new() -> Self
    {
        Material{color: create_color(1.0, 1.0, 1.0), ambient: 0.1, diffuse: 0.9,
            specular: 0.9, shininess: 200.0, pattern: None}
    }

    pub fn lighting(&self, object: Shape, light: PointLight,
        point: Tuple, eyev: Tuple,
        normalv: Tuple, in_shadow: bool) -> Tuple
    {
        let color = match &self.pattern
        {
            Some(p) => match p
            {
                Pattern::StripePattern(s) => s.stripe_at_object(object, point),
                Pattern::TestPattern(t) => t.pattern_at(point),
                Pattern::GradientPattern(_g) => p.pattern_at_shape(object, point),
                Pattern::RingPattern(_r) => p.pattern_at_shape(object, point),
                Pattern::CheckerPattern(_c) => p.pattern_at_shape(object, point),
            },
            None => self.color,
        };

        // combine the surface color with the light's color/intensity
        let effective_color = color.hadamard_product(light.intensity);

        // find the direction to the light source
        let lightv = light.position.sub(point).normalize();

        // compute the ambient contribution
        let ambient = effective_color.multiply(self.ambient);

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot_product(normalv);
        let diffuse: Tuple;
        let specular: Tuple;
        let color_black = create_color(0.0, 0.0, 0.0);
        if in_shadow
        {
            diffuse = color_black;
            specular = color_black;
        }
        else if light_dot_normal < 0.0
        {
            diffuse = color_black;
            specular = color_black;
        }
        else
        {
            // compute the diffuse contribution
            diffuse = effective_color.multiply(self.diffuse).multiply(light_dot_normal);

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = lightv.negate().reflect(normalv);
            let reflect_dot_eye = reflectv.dot_product(eyev);

            if reflect_dot_eye <= 0.0
            {
                specular = color_black;
            }
            else
            {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity.multiply(self.specular).multiply(factor);
            }
        }

        // Add the three contributions together to get the final shading
        ambient.add(diffuse).add(specular)
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

        // p.86 Scenario: Lighting with the eye between the light and the surface
        let sphere2 = Shape::new_sphere(2);
        let material2 = Material::new();
        let position2 = create_point(0.0, 0.0, 0.0);
        let eyev2 = create_vector(0.0, 0.0, -1.0);
        let normalv2 = create_vector(0.0, 0.0, -1.0);
        let light2 = PointLight::new(create_point(0.0, 0.0, -10.0), create_color(1.0, 1.0, 1.0));
        let result2 = material2.lighting(sphere2,
            light2, position2, eyev2, normalv2, false);
        assert_eq!(result2, create_color(1.9, 1.9, 1.9));

        // p.86 Scenario: Lighting with the eye between the light and the surface, eye offset 45 degrees
        let sphere3 = Shape::new_sphere(3);
        let material3 = Material::new();
        let position3 = create_point(0.0, 0.0, 0.0);
        let sqrt2 = 2.0_f64.sqrt();
        let eyev3 = create_vector(0.0, sqrt2 / 2.0, -sqrt2 / 2.0);
        let normalv3 = create_vector(0.0, 0.0, -1.0);
        let light3 = PointLight::new(create_point(0.0, 0.0, -10.0), create_color(1.0, 1.0, 1.0));
        let result3 = material3.lighting(sphere3,
            light3, position3, eyev3, normalv3, false);
        assert_eq!(result3, create_color(1.0, 1.0, 1.0));

        // p.87 Scenario: Lighting with eye opposite surface, light offset 45 degrees
        let sphere4 = Shape::new_sphere(4);
        let material4 = Material::new();
        let position4 = create_point(0.0, 0.0, 0.0);
        let eyev4 = create_vector(0.0, 0.0, -1.0);
        let normalv4 = create_vector(0.0, 0.0, -1.0);
        let light4 = PointLight::new(create_point(0.0, 10.0, -10.0), create_color(1.0, 1.0, 1.0));
        let result4 = material4.lighting(sphere4,
            light4, position4, eyev4, normalv4, false);
        assert_eq!(result4, create_color(0.7364, 0.7364, 0.7364));

        // p.87 Scenario: Lighting with eye in the path of the reflection vector
        let sphere5 = Shape::new_sphere(5);
        let material5 = Material::new();
        let position5 = create_point(0.0, 0.0, 0.0);
        let eyev5 = create_vector(0.0, -sqrt2 / 2.0, -sqrt2 / 2.0);
        let normalv5 = create_vector(0.0, 0.0, -1.0);
        let light5 = PointLight::new(create_point(0.0, 10.0, -10.0), create_color(1.0, 1.0, 1.0));
        let result5 = material5.lighting(sphere5,
            light5, position5, eyev5, normalv5, false);
        assert_eq!(result5, create_color(1.6364, 1.6364, 1.6364));

        // p.88 Scenario: Lighting with the light behind the surface
        let sphere6 = Shape::new_sphere(6);
        let material6 = Material::new();
        let position6 = create_point(0.0, 0.0, 0.0);
        let eyev6 = create_vector(0.0, 0.0, -1.0);
        let normalv6 = create_vector(0.0, 0.0, -1.0);
        let light6 = PointLight::new(create_point(0.0, 0.0, 10.0), create_color(1.0, 1.0, 1.0));
        let result6 = material6.lighting(sphere6, light6, position6, eyev6, normalv6, false);
        assert_eq!(result6, create_color(0.1, 0.1, 0.1));

        // p.110 Scenario: Lighting with the surface in shadow
        let sphere7 = Shape::new_sphere(7);
        let material7 = Material::new();
        let position7 = create_point(0.0, 0.0, 0.0);
        let eyev7 = create_vector(0.0, 0.0, -1.0);
        let normalv7 = create_vector(0.0, 0.0, -1.0);
        let light7 = PointLight::new(create_point(0.0, 0.0, -10.0), create_color(1.0, 1.0, 1.0));
        let in_shadow7 = true;
        let result7 = material7.lighting(sphere7, light7, position7, eyev7, normalv7, in_shadow7);
        assert_eq!(result7, create_color(0.1, 0.1, 0.1));
    }
}
