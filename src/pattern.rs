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
}

impl StripePattern
{
    pub fn new(a: Tuple, b: Tuple) -> StripePattern
    {
        StripePattern{a: a, b: b}
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

    pub fn pattern_at(&self, point: Tuple) -> Tuple
    {
        self.stripe_at(point)
    }
}

#[derive(Clone, Debug)]
pub struct TestPattern
{
}

impl TestPattern
{
    pub fn new() -> TestPattern
    {
        TestPattern{}
    }

    pub fn pattern_at(&self, point: Tuple) -> Tuple
    {
        let v = point.get_vec();
        create_color(v[0], v[1], v[2])
    }
}

#[derive(Clone, Debug)]
pub struct GradientPattern
{
    pub a: Tuple,
    pub b: Tuple,
}

impl GradientPattern
{
    pub fn new(a: Tuple, b: Tuple) -> GradientPattern
    {
        GradientPattern{a: a, b: b}
    }

    pub fn pattern_at(&self, point: Tuple) -> Tuple
    {
        let distance = self.b.sub(self.a);
        let x = point.get_vec()[0];
        let fraction = x - x.floor();
        self.a.add(distance.multiply(fraction))
    }
}

#[derive(Clone, Debug)]
pub struct RingPattern
{
    pub a: Tuple,
    pub b: Tuple,
}

impl RingPattern
{
    pub fn new(a: Tuple, b: Tuple) -> RingPattern
    {
        RingPattern{a: a, b: b}
    }

    pub fn pattern_at(&self, point: Tuple) -> Tuple
    {
        let x = point.get_vec()[0];
        let z = point.get_vec()[2];
        if ((x * x) + (z * z)).sqrt().floor().rem_euclid(2.0_f64) < 1.0
        {
            self.a
        }
        else
        {
            self.b
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckerPattern
{
    pub a: Tuple,
    pub b: Tuple,
}

impl CheckerPattern
{
    pub fn new(a: Tuple, b: Tuple) -> CheckerPattern
    {
        CheckerPattern{a: a, b: b}
    }

    pub fn pattern_at(&self, point: Tuple) -> Tuple
    {
        let v = point.get_vec();
        let sum = v[0].floor() + v[1].floor() + v[2].floor();
        if sum.rem_euclid(2.0_f64) < 1.0
        {
            self.a
        }
        else
        {
            self.b
        }
    }
}

#[derive(Clone, Debug)]
pub enum PatternCommon
{
    StripePattern(StripePattern),
    TestPattern(TestPattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    CheckerPattern(CheckerPattern),
}

#[derive(Clone, Debug)]
pub struct Pattern
{
    transform: Matrix,
    common: PatternCommon,
}

impl Pattern
{
    pub fn get_common(&self) -> PatternCommon
    {
        self.common.clone()
    }

    pub fn new_stripe_pattern(a: Tuple, b: Tuple) -> Pattern
    {
        Pattern{transform: Matrix::identity(4),
            common: PatternCommon::StripePattern(StripePattern::new(a, b))}
    }

    pub fn test_pattern() -> Pattern
    {
        Pattern{transform: Matrix::identity(4),
            common: PatternCommon::TestPattern(TestPattern::new())}
    }

    pub fn new_gradient_pattern(a: Tuple, b: Tuple) -> Pattern
    {
        Pattern{transform: Matrix::identity(4),
            common: PatternCommon::GradientPattern(GradientPattern::new(a, b))}
    }

    pub fn new_ring_pattern(a: Tuple, b: Tuple) -> Pattern
    {
        Pattern{transform: Matrix::identity(4),
            common: PatternCommon::RingPattern(RingPattern::new(a, b))}
    }

    pub fn new_checker_pattern(a: Tuple, b: Tuple) -> Pattern
    {
        Pattern{transform: Matrix::identity(4),
            common: PatternCommon::CheckerPattern(CheckerPattern::new(a, b))}
    }

    pub fn get_pattern_transform(&self) -> Matrix
    {
        self.transform.clone()
    }

    pub fn set_pattern_transform(&mut self, transform: Matrix)
    {
        self.transform = transform;
    }

    pub fn pattern_at_shape(&self, shape: Shape, world_point: Tuple) -> Tuple
    {
        let object_point = shape.get_transform().inverse().multiply_tuple(world_point);
        let pattern_point = self.get_pattern_transform().inverse().multiply_tuple(object_point);
        match &self.common
        {
            PatternCommon::StripePattern(s) => s.pattern_at(pattern_point),
            PatternCommon::TestPattern(t) => t.pattern_at(pattern_point),
            PatternCommon::GradientPattern(g) => g.pattern_at(pattern_point),
            PatternCommon::RingPattern(r) => r.pattern_at(pattern_point),
            PatternCommon::CheckerPattern(c) => c.pattern_at(pattern_point),
        }
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
        let s5 = Shape::new_sphere(5);
        m5.pattern = Some(Pattern::new_stripe_pattern(white, black));
        m5.ambient = 1.0;
        m5.diffuse = 0.0;
        m5.specular = 0.0;
        let eyev5 = create_vector(0.0, 0.0, -1.0);
        let normalv5 = create_vector(0.0, 0.0, -1.0);
        let light5 = PointLight::new(create_point(0.0, 0.0, -10.0),
            create_color(1.0, 1.0, 1.0));
        let c51 = m5.lighting(s5.clone(), light5, create_point(0.9, 0.0, 0.0),
            eyev5, normalv5, false);
        let c52 = m5.lighting(s5.clone(), light5, create_point(1.1, 0.0, 0.0),
            eyev5, normalv5, false);
        assert_eq!(c51, white);
        assert_eq!(c52, black);

        // p.131 Scenario: Stripes with an object transformation
        let mut m6 = Material::new();
        let mut s6 = Shape::new_sphere(6);
        s6.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let p6 = Pattern::new_stripe_pattern(white, black);
        m6.pattern = Some(p6.clone());
        s6.set_material(m6);
        let c6 = p6.pattern_at_shape(s6, create_point(1.5, 0.0, 0.0));
        assert_eq!(c6, white);

        // p.131 Scenario: Stripes with a pattern transformation
        let mut m7 = Material::new();
        let mut s7 = Shape::new_sphere(7);
        let mut p7 = Pattern::new_stripe_pattern(white, black);
        p7.set_pattern_transform(Matrix::scaling(2.0, 2.0, 2.0));
        m7.pattern = Some(p7.clone());
        s7.set_material(m7);
        let c7 = p7.pattern_at_shape(s7, create_point(1.5, 0.0, 0.0));
        assert_eq!(c7, white);

        // p.131 Scenario: Stripes with both an object and a pattern transformation
        let mut m8 = Material::new();
        let mut s8 = Shape::new_sphere(8);
        s8.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut p8 = Pattern::new_stripe_pattern(white, black);
        p8.set_pattern_transform(Matrix::translation(0.5, 0.0, 0.0));
        m8.pattern = Some(p8.clone());
        s8.set_material(m8);
        let c8 = p8.pattern_at_shape(s8, create_point(2.5, 0.0, 0.0));
        assert_eq!(c8, white);

        // p.133 Scenario: The default pattern transformation
        let p9 = Pattern::test_pattern();
        assert_eq!(p9.get_pattern_transform(), Matrix::identity(4));

        // p.133 Scenario: Assigning a transformation
        let mut p10 = Pattern::test_pattern();
        p10.set_pattern_transform(Matrix::translation(1.0, 2.0, 3.0));
        assert_eq!(p10.get_pattern_transform(), Matrix::translation(1.0, 2.0, 3.0));

        // p.134 Scenario: A pattern with an object transformation
        let mut s11 = Shape::new_sphere(11);
        s11.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let p11 = Pattern::test_pattern();
        let c11 = p11.pattern_at_shape(s11, create_point(2.0, 3.0, 4.0));
        assert_eq!(c11, create_color(1.0, 1.5, 2.0));

        // p.134 Scenario: A pattern with a pattern transformation
        let s12 = Shape::new_sphere(12);
        let mut p12 = Pattern::test_pattern();
        p12.set_pattern_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let c12 = p12.pattern_at_shape(s12, create_point(2.0, 3.0, 4.0));
        assert_eq!(c12, create_color(1.0, 1.5, 2.0));

        // p.134 Scenario: A pattern with both an object and pattern transformation
        let mut s13 = Shape::new_sphere(13);
        s13.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut p13 = Pattern::test_pattern();
        p13.set_pattern_transform(Matrix::translation(0.5, 1.0, 1.5));
        let c13 = p13.pattern_at_shape(s13, create_point(2.5, 3.0, 3.5));
        assert_eq!(c13, create_color(0.75, 0.5, 0.25));

        // p.135 Scenario: A gradient linearly interpolates between colors
        let p14 = GradientPattern::new(white, black);
        assert_eq!(p14.pattern_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p14.pattern_at(create_point(0.25, 0.0, 0.0)), create_color(0.75, 0.75, 0.75));
        assert_eq!(p14.pattern_at(create_point(0.5, 0.0, 0.0)), create_color(0.5, 0.5, 0.5));
        assert_eq!(p14.pattern_at(create_point(0.75, 0.0, 0.0)), create_color(0.25, 0.25, 0.25));

        // p.136 Scenario: A ring should extend in both x and z
        let p15 = RingPattern::new(white, black);
        assert_eq!(p15.pattern_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p15.pattern_at(create_point(1.0, 0.0, 0.0)), black);
        assert_eq!(p15.pattern_at(create_point(0.0, 0.0, 1.0)), black);
        assert_eq!(p15.pattern_at(create_point(0.708, 0.0, 0.708)), black);

        // p.137 Scenario: Checkers should repeat in x
        let p16 = CheckerPattern::new(white, black);
        assert_eq!(p16.pattern_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p16.pattern_at(create_point(0.99, 0.0, 0.0)), white);
        assert_eq!(p16.pattern_at(create_point(1.01, 0.0, 0.0)), black);

        // p.137 Scenario: Checkers should repeat in y
        let p17 = CheckerPattern::new(white, black);
        assert_eq!(p17.pattern_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p17.pattern_at(create_point(0.0, 0.99, 0.0)), white);
        assert_eq!(p17.pattern_at(create_point(0.0, 1.01, 0.0)), black);

        // p.137 Scenario: Checkers should repeat in z
        let p18 = CheckerPattern::new(white, black);
        assert_eq!(p18.pattern_at(create_point(0.0, 0.0, 0.0)), white);
        assert_eq!(p18.pattern_at(create_point(0.0, 0.0, 0.99)), white);
        assert_eq!(p18.pattern_at(create_point(0.0, 0.0, 1.01)), black);
    }
}
