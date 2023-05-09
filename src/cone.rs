use std::fmt;
use std::cmp;
use std::f64::consts::PI;
use crate::arithmetic::*;
use crate::intersections::*;
use crate::tuple::*;
use crate::ray::*;
use crate::matrix::*;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct Cone
{
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

// A double-napped cone following the y axis, extending towards infinity
impl Cone
{
    pub fn new() -> Self
    {
        Cone{minimum: f64::NEG_INFINITY, maximum: f64::INFINITY,
            closed: false}
    }

    // a helper function to reduce duplication.
    // checks to see if the intersection as `t` is within a radius
    // of the cone from the y axis.
    fn check_cap(&self, ray: Ray, t: f64, radius: f64) -> bool
    {
        let vo = ray.origin.get_vec();
        let vd = ray.direction.get_vec();
        let x = vo[0] + t * vd[0];
        let z = vo[2] + t * vd[2];
        let dist_squared = (x * x) + (z * z);
        dist_squared <= radius * radius
    }

    fn intersect_caps(&self, ray: Ray) -> Vec<f64>
    {
        let mut xs = Vec::new();

        // caps only matter if the cylinder is closed, and might possibly be
        // intersected by the ray.
        if (!self.closed) || fuzzy_equal(ray.direction.get_vec()[1], 0.0)
        {
            return xs;
        }

        // check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cly.minimum
        let t0 = (self.minimum - ray.origin.get_vec()[1]) / ray.direction.get_vec()[1];
        if self.check_cap(ray, t0, self.minimum)
        {
            xs.push(t0);
        }

        // check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cly.maximum
        let t1 = (self.maximum - ray.origin.get_vec()[1]) / ray.direction.get_vec()[1];
        if self.check_cap(ray, t1, self.maximum)
        {
            xs.push(t1);
        }
        return xs;
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64>
    {
        let vd = ray.direction.get_vec();
        let a = (vd[0] * vd[0]) - (vd[1] * vd[1]) + (vd[2] * vd[2]);

        let vo = ray.origin.get_vec();
        let b = 2.0 * vo[0] * vd[0] - 2.0 * vo[1] * vd[1] + 2.0 * vo[2] * vd[2];
        let c = (vo[0] * vo[0]) - (vo[1] * vo[1]) + (vo[2] * vo[2]);
        let disc = (b * b) - 4.0 * a * c;

        let mut xs = Vec::new();

        if fuzzy_equal(a, 0.0)
        {
            if fuzzy_equal(b, 0.0)
            {
                return Vec::new();
            }
            let t = -c / (2.0 * b);
            xs.push(t);
        }

        // ray does not intersect the cone
        if disc < 0.0
        {
            return self.intersect_caps(ray);
        }

        let mut t0 = ((-b) - disc.sqrt()) / (2.0 * a);
        let mut t1 = ((-b) + disc.sqrt()) / (2.0 * a);

        if t0 > t1
        {
            let swap = t0;
            t0 = t1;
            t1 = swap;
        }

        let y0 = vo[1] + t0 * vd[1];
        if self.minimum < y0 && y0 < self.maximum
        {
            xs.push(t0);
        }

        let y1 = vo[1] + t1 * vd[1];
        if self.minimum < y1 && y1 < self.maximum
        {
            xs.push(t1);
        }

        let mut caps = self.intersect_caps(ray);
        xs.append(&mut caps);

        return xs;
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple
    {
        // compute the square of the distance from the y axis
        let v = point.get_vec();
        let dist = (v[0] * v[0]) + (v[2] * v[2]);

        if dist < self.maximum.powi(2) && v[1] >= self.maximum - EPSILON
        {
            return create_vector(0.0, 1.0, 0.0);
        }
        else if dist < self.minimum.powi(2) && v[1] <= self.minimum + EPSILON
        {
            return create_vector(0.0, -1.0, 0.0);
        }
        else
        {
            let mut y = ((v[0] * v[0]) + (v[2] * v[2])).sqrt();
            if v[1] > 0.0
            {
                y = -y;
            }
            return create_vector(v[0], y, v[2]);
        }
    }
}

impl fmt::Display for Cone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "cone")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_cones_feature1()
    {
        // p.189 Scenario: Intersecting a cone with a ray
        let c1 = Cone::new();
        let origins1 = vec![create_point(0.0, 0.0, -5.0),
            create_point(0.0, 0.0, -5.0),
            create_point(1.0, 1.0, -5.0)];
        let directions1 = vec![create_vector(0.0, 0.0, 1.0),
            create_vector(1.0, 1.0, 1.0),
            create_vector(-0.5, -1.0, 1.0)];
        let t10 = vec![5.0, 8.66025, 4.55006];
        let t11 = vec![5.0, 8.66025, 49.44994];

        for i in 0..origins1.len()
        {
            let r1 = Ray::new(origins1[i], directions1[i].normalize());
            let xs1 = c1.local_intersect(r1);
            assert_eq!(xs1.len(), 2);
            assert!(fuzzy_equal(xs1[0], t10[i]));
            assert!(fuzzy_equal(xs1[1], t11[i]));
        }
    }

    #[test]
    fn test_cones_feature2()
    {
        // p.190 Scenario: Intersecting a cone with a ray parallel to one of its halves
        let c2 = Cone::new();
        let direction2 = create_vector(0.0, 1.0, 1.0).normalize();
        let r2 = Ray::new(create_point(0.0, 0.0, -1.0), direction2);
        let xs2 = c2.local_intersect(r2);
        assert_eq!(xs2.len(), 1);
        assert!(fuzzy_equal(xs2[0], 0.35355));
    }

    #[test]
    fn test_cones_feature3()
    {
        // p.190 Scenario: Intersecting a cone's end caps
        let mut c3 = Cone::new();
        c3.minimum = -0.5;
        c3.maximum = 0.5;
        c3.closed = true;

        let origins3 = vec![create_point(0.0, 0.0, -5.0),
            create_point(0.0, 0.0, -0.25),
            create_point(0.0, 0.0, -0.25)];
        let directions3 = vec![create_vector(0.0, 1.0, 0.0),
            create_vector(0.0, 1.0, 1.0),
            create_vector(0.0, 1.0, 0.0)];
        let counts3 = vec![0, 2, 4];

        for i in 0..origins3.len()
        {
            let r3 = Ray::new(origins3[i], directions3[i].normalize());
            let xs3 = c3.local_intersect(r3);
            assert_eq!(xs3.len(), counts3[i]);
        }
    }

    #[test]
    fn test_cones_feature4()
    {
        // p.190 Scenario: Computing a normal vector on a cone
        let mut c4 = Cone::new();

        let points4 = vec![create_point(0.0, 0.0, 0.0),
            create_point(1.0, 1.0, 1.0),
            create_point(-1.0, -1.0, 0.0)];
        let sqrt2 = 2.0_f64.sqrt();
        let normals4 = vec![create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, -sqrt2, 1.0),
            create_vector(-1.0, 1.0, 0.0)];

        for i in 0..points4.len()
        {
            let n4 = c4.local_normal_at(points4[i]);
            assert_eq!(n4, normals4[i]);
        }
    }
}
