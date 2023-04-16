use crate::material::*;
use crate::matrix::*;
use crate::tuple::*;
use crate::pointlight::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct StripePattern
{
    pub a: Tuple,
    pub b: Tuple,
    pub transform: Matrix,
}

impl StripePattern
{
    pub fn new(a: Tuple, b: Tuple) -> StripePattern
    {
        StripePattern{a: a, b: b, transform: Matrix::identity(4)}
    }

    pub fn stripe_at(&self, point: Tuple) -> Tuple
    {
        let x = point.get_vec()[0].floor();
        if x.rem_euclid(2.0_f64) < 1.0
        {
            self.a
        }
        else
        {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: Shape, world_point: Tuple) -> Tuple
    {
        let object_point = object.get_transform().inverse().multiply_tuple(world_point);
        let pattern_point = self.transform.inverse().multiply_tuple(object_point);
        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_pattern_feature()
    {
        let black = create_color(0.0, 0.0, 0.0);
        let white = create_color(1.0, 1.0, 1.0);

        // p.128 Scenario: Creating a stripe pattern
        let p1 = StripePattern::new(white, black);
        assert_eq!(p1.a, white);
        assert_eq!(p1.b, black);

        // p.129 Scenario: A stripe pattern is constant in y
        let p2 = StripePattern::new(white, black);
        assert_eq!(p2.stripe_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p2.stripe_at(create_point(0.0, 1.0, 0.0)), white);
        assert_eq!(p2.stripe_at(create_point(0.0, 2.0, 0.0)), white);

        // p.129 Scenario: A stripe pattern is constant in z
        let p3 = StripePattern::new(white, black);
        assert_eq!(p3.stripe_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p3.stripe_at(create_point(0.0, 0.0, 1.0)), white);
        assert_eq!(p3.stripe_at(create_point(0.0, 0.0, 2.0)), white);

        // p.129 Scenario: A stripe pattern alternates in x
        let p4 = StripePattern::new(white, black);
        assert_eq!(p4.stripe_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p4.stripe_at(create_point(0.9, 0.0, 0.0)), white);
        assert_eq!(p4.stripe_at(create_point(1.0, 0.0, 0.0)), black);
        assert_eq!(p4.stripe_at(create_point(-0.1, 0.0, 0.0)), black);
        assert_eq!(p4.stripe_at(create_point(-1.0, 0.0, 0.0)), black);
        assert_eq!(p4.stripe_at(create_point(-1.1, 0.0, 0.0)), white);

        // p.129 Scenario: Lighting with a pattern applied
        let mut m5 = Material::new();
        m5.pattern = Some(StripePattern::new(white, black));
        m5.ambient = 1.0;
        m5.diffuse = 0.0;
        m5.specular = 0.0;
        let eyev5 = create_vector(0.0, 0.0, -1.0);
        let normalv5 = create_vector(0.0, 0.0, -1.0);
        let light5 = PointLight::new(create_point(0.0, 0.0, -10.0),
            create_color(1.0, 1.0, 1.0));
        let c51 = m5.lighting(light5, create_point(0.9, 0.0, 0.0),
            eyev5, normalv5, false);
        let c52 = m5.lighting(light5, create_point(1.1, 0.0, 0.0),
            eyev5, normalv5, false);
        assert_eq!(c51, white);
        assert_eq!(c52, black);

        // p.131 Scenario: Stripes with an object transformation
        let mut m6 = Material::new();
        let mut s6 = Shape::new_sphere(6);
        s6.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let p6 = StripePattern::new(white, black);
        m6.pattern = Some(p6.clone());
        s6.set_material(m6);
        let c6 = p6.stripe_at_object(s6, create_point(1.5, 0.0, 0.0));
        assert_eq!(c6, white);

        // p.131 Scenario: Stripes with a pattern transformation
        let mut m7 = Material::new();
        let mut s7 = Shape::new_sphere(7);
        let mut p7 = StripePattern::new(white, black);
        p7.transform = Matrix::scaling(2.0, 2.0, 2.0);
        m7.pattern = Some(p7.clone());
        s7.set_material(m7);
        let c7 = p7.stripe_at_object(s7, create_point(1.5, 0.0, 0.0));
        assert_eq!(c7, white);

        // p.131 Scenario: Stripes with both an object and a pattern transformation
        let mut m8 = Material::new();
        let mut s8 = Shape::new_sphere(8);
        s8.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut p8 = StripePattern::new(white, black);
        p8.transform = Matrix::translation(0.5, 0.0, 0.0);
        m8.pattern = Some(p8.clone());
        s8.set_material(m8);
        let c8 = p8.stripe_at_object(s8, create_point(2.5, 0.0, 0.0));
        assert_eq!(c8, white);
    }
}
