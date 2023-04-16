use crate::tuple::*;

#[derive(Copy, Clone, Debug)]
pub struct StripePattern
{
    pub a: Tuple,
    pub b: Tuple,
}

impl StripePattern
{
    pub fn new(a: Tuple, b: Tuple) -> StripePattern
    {
        StripePattern{a, b}
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
    }
}
