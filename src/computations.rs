use crate::arithmetic::*;
use crate::intersections::*;
use crate::ray::*;
use crate::shape::*;
use crate::sphere::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct Computations
{
    pub t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub reflectv: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl Computations
{
    pub fn new(t: f64, object: Shape, point: Tuple, eyev: Tuple,
         normalv: Tuple, inside: bool, over_point: Tuple,
         under_point: Tuple, reflectv: Tuple, n1: f64, n2: f64) -> Self
    {
        Computations{t, object, point, eyev, normalv, inside, over_point,
            under_point, reflectv, n1, n2}
    }

    pub fn schlick(&self) -> f64
    {
        // find the cosine of the angle between the eye and normal vectors
        let mut cos = self.eyev.dot_product(self.normalv);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2
        {
            let n = self.n1 / self.n2;
            let sin2_t = (n * n) * (1.0 - (cos * cos));
            if sin2_t > 1.0
            {
                return 1.0;
            }

            // compute cosine of theta_t using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();

            // wnen n1 > n2, use cos(theta_t) instead
            cos = cos_t
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.0);
        return r0 + (1.0 - r0) * (1.0 - cos).powf(5.0);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_computations_schlick_feature()
    {
        // p.161 Scenario: The Schlick approximation under total internal reflection
        let shape1 = Shape::glass_sphere(1);
        let sqrt2 = 2.0_f64.sqrt();
        let ray1 = Ray::new(create_point(0.0, 0.0, sqrt2 / 2.0),
            create_vector(0.0, 1.0, 0.0));
        let i11 = Intersection::new(-sqrt2 / 2.0, shape1.clone());
        let i12 = Intersection::new(sqrt2 / 2.0, shape1.clone());
        let xs1 = Intersections::new(vec![i11.clone(), i12.clone()]);
        let comps1 = i12.prepare_computations(ray1, xs1);
        let reflectance1 = comps1.schlick();
        assert!(fuzzy_equal(reflectance1, 1.0));

        // p.162 Scenario: The Schlick approximation with a perpendicular viewing angle
        let shape2 = Shape::glass_sphere(2);
        let ray2 = Ray::new(create_point(0.0, 0.0, 0.0),
            create_vector(0.0, 1.0, 0.0));
        let i21 = Intersection::new(-1.0, shape2.clone());
        let i22 = Intersection::new(1.0, shape2.clone());
        let xs2 = Intersections::new(vec![i21.clone(), i22.clone()]);
        let comps2 = i22.prepare_computations(ray2, xs2);
        let reflectance2 = comps2.schlick();
        assert!(fuzzy_equal(reflectance2, 0.04));

        // p.163 Scenario: The Schlick approximation with small angle and n2 > n1
        let shape3 = Shape::glass_sphere(3);
        let ray3 = Ray::new(create_point(0.0, 0.99, -2.0),
            create_vector(0.0, 0.0, 1.0));
        let i31 = Intersection::new(1.8589, shape3.clone());
        let xs3 = Intersections::new(vec![i31.clone()]);
        let comps3 = i31.prepare_computations(ray3, xs3);
        let reflectance3 = comps3.schlick();
        assert!(fuzzy_equal(reflectance3, 0.48873));
    }
}
