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
        let cos = self.eyev.dot_product(self.normalv);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2
        {
            let n = self.n1 / self.n2;
            let sin2_t = (n * n) * (1.0 - (cos * cos));
            if sin2_t > 1.0
            {
                return 1.0;
            }
        }

        // return anything but 1.0 here, so that the test will fail
        // appropriately if something goes wrong.
        return 0.0;
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
        let reflectance = comps1.schlick();
        assert!(fuzzy_equal(reflectance, 1.0));
    }
}
