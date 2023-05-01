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
pub struct Cube
{
    id: i32,
    transform: Matrix,
    material: Material,
    saved_ray: Ray,
}

// An axis aligned bounding box from -1 to +1 on each axis
impl Cube
{
    pub fn get_local_transform(&self) -> Matrix
    {
        self.transform.clone()
    }

    pub fn set_local_transform(&mut self, transform: Matrix)
    {
        self.transform = transform;
    }

    pub fn get_local_material(&self) -> Material
    {
        self.material.clone()
    }

    pub fn set_local_material(&mut self, material: Material)
    {
        self.material = material;
    }

    pub fn get_id(&self) -> i32
    {
        self.id
    }
}

impl Cube
{
    pub fn new(id: i32) -> Self
    {
        let zero_point = create_point(0.0, 0.0, 0.0);
        let zero_vector = create_vector(0.0, 0.0, 0.0);
        Cube{id: id, transform: Matrix::identity(4),
            material: Material::new(),
            saved_ray: Ray::new(zero_point, zero_vector)}
    }

    fn check_axis(&mut self, origin: f64, direction: f64) -> (f64, f64)
    {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let mut tmin;
        let mut tmax;
        if direction.abs() >= EPSILON
        {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        }
        else
        {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax
        {
            let t = tmin;
            tmin = tmax;
            tmax = t;
        }

        (tmin, tmax)
    }

    fn max3(&self, a: f64, b: f64, c: f64) -> f64
    {
        let mut n = a;
        if b > n
        {
            n = b;
        }
        if c > n
        {
            n = c;
        }
        n
    }

    fn min3(&self, a: f64, b: f64, c: f64) -> f64
    {
        let mut n = a;
        if b < n
        {
            n = b;
        }
        if c < n
        {
            n = c;
        }
        n
    }

    pub fn local_intersect(&mut self, ray: Ray) -> Vec<f64>
    {
        let (xtmin, xtmax) = self.check_axis(ray.origin.get_vec()[0],
            ray.direction.get_vec()[0]);
        let (ytmin, ytmax) = self.check_axis(ray.origin.get_vec()[1],
            ray.direction.get_vec()[1]);
        let (ztmin, ztmax) = self.check_axis(ray.origin.get_vec()[2],
            ray.direction.get_vec()[2]);

        let tmin = self.max3(xtmin, ytmin, ztmin);
        let tmax = self.min3(xtmax, ytmax, ztmax);

        if tmin > tmax
        {
            return vec![];
        }

        return vec![tmin, tmax];
    }

    pub fn local_get_saved_ray(&self) -> Ray
    {
        self.saved_ray
    }

    pub fn local_set_saved_ray(&mut self, saved_ray: Ray)
    {
        self.saved_ray = saved_ray;
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple
    {
        let v = point.get_vec();
        let x = v[0];
        let y = v[1];
        let z =  v[2];
        let maxc = self.max3(x.abs(), y.abs(), z.abs());

        if maxc == x.abs()
        {
            return create_vector(x, 0.0, 0.0)
        }
        else if maxc == y.abs()
        {
            return create_vector(0.0, y, 0.0)
        }
        return create_vector(0.0, 0.0, z);
    }
}

impl PartialEq for Cube
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}

impl fmt::Display for Cube
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "cube {}", self.id)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_cubes_feature1()
    {
        // p.168 Scenario: A ray intersects a cube
        let mut c1 = Cube::new(1);
        let origins1 = vec![create_point(5.0, 0.5, 0.0),
            create_point(-5.0, 0.5, 0.0),
            create_point(0.5, 5.0, 0.0),
            create_point(0.5, -5.0, 0.0),
            create_point(0.5, 0.0, 5.0),
            create_point(0.5, 0.0, -5.0),
            create_point(0.0, 0.5, 0.0)];
        let directions1 = vec![create_vector(-1.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
            create_vector(0.0, -1.0, 0.0),
            create_vector(0.0, 1.0, 0.0),
            create_vector(0.0, 0.0, -1.0),
            create_vector(0.0, 0.0, 1.0),
            create_vector(0.0, 0.0, 1.0)];
        let t11 = vec![4.0, 4.0, 4.0, 4.0, 4.0, 4.0, -1.0];
        let t21 = vec![6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 1.0];

        for i in 0..origins1.len()
        {
            let r1 = Ray::new(origins1[i], directions1[i]);
            let xs1 = c1.local_intersect(r1);
            assert_eq!(xs1.len(), 2);
            assert!(fuzzy_equal(xs1[0], t11[i]));
            assert!(fuzzy_equal(xs1[1], t21[i]));
        }
    }

    #[test]
    fn test_cubes_feature2()
    {
        // p.172 Scenario: A ray misses a cube
        let mut c2 = Cube::new(2);
        let origins2 = vec![create_point(-2.0, 0.0, 0.0),
            create_point(0.0, -2.0, 0.0),
            create_point(0.0, 0.0, -2.0),
            create_point(2.0, 0.0, 2.0),
            create_point(0.0, 2.0, 2.0),
            create_point(2.0, 2.0, 0.0)];
        let directions2 = vec![create_vector(0.2673, 0.5345, 0.8018),
            create_vector(0.8018, 0.2673, 0.5345),
            create_vector(0.5345, 0.8018, 0.2673),
            create_vector(0.0, 0.0, -1.0),
            create_vector(0.0, -1.0, 0.0),
            create_vector(-1.0, 0.0, 0.0)];

        for i in 0..origins2.len()
        {
            let r2 = Ray::new(origins2[i], directions2[i]);
            let xs2 = c2.local_intersect(r2);
            assert_eq!(xs2.len(), 0);
        }
    }

    #[test]
    fn test_cubes_feature3()
    {
        // p.172 Scenario: The normal on the surface of a cube
        let mut c3 = Cube::new(3);
        let points3 = vec![create_point(1.0, 0.5, -0.8),
            create_point(-1.0, -0.2, 0.9),
            create_point(-0.4, 1.0, -0.1),
            create_point(0.3, -1.0, -0.7),
            create_point(-0.6, 0.3, 1.0),
            create_point(0.4, 0.4, -1.0),
            create_point(1.0, 1.0, 1.0),
            create_point(-1.0, -1.0, -1.0)];
        let normals3 = vec![create_vector(1.0, 0.0, 0.0),
            create_vector(-1.0, 0.0, 0.0),
            create_vector(0.0, 1.0, 0.0),
            create_vector(0.0, -1.0, 0.0),
            create_vector(0.0, 0.0, 1.0),
            create_vector(0.0, 0.0, -1.0),
            create_vector(1.0, 0.0, 0.0),
            create_vector(-1.0, 0.0, 0.0)];

        for i in 0..points3.len()
        {
            let normal = c3.local_normal_at(points3[i]);
            assert_eq!(normal, normals3[i]);
        }
    }
}
