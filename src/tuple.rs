use std::fmt;
use crate::arithmetic::*;

#[derive(Copy, Clone)]
pub struct Tuple
{
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple
{
    pub fn get_vec(self) -> Vec<f64>
    {
        vec![self.x, self.y, self.z, self.w]
    }

    pub fn add(&self, b: Tuple) -> Tuple
    {
        Tuple{x: self.x + b.x, y: self.y + b.y, z: self.z + b.z, w: self.w + b.w}
    }

    pub fn sub(&self, b: Tuple) -> Tuple
    {
        Tuple{x: self.x - b.x, y: self.y - b.y, z: self.z - b.z, w: self.w - b.w}
    }

    pub fn negate(&self) -> Tuple
    {
        Tuple{x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }

    pub fn multiply(&self, scalar: f64) -> Tuple
    {
        Tuple{x: self.x * scalar, y: self.y * scalar, z: self.z * scalar, w: self.w * scalar}
    }

    pub fn divide(&self, scalar: f64) -> Tuple
    {
        Tuple{x: self.x / scalar, y: self.y / scalar, z: self.z / scalar, w: self.w / scalar}
    }

    pub fn magnitude(&self) -> f64
    {
        let n = (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);
        n.sqrt()
    }

    pub fn normalize(&self) -> Tuple
    {
        let m = self.magnitude();
        Tuple{x: self.x / m, y: self.y / m, z: self.z / m, w: self.w / m}
    }

    pub fn dot_product(&self, b: Tuple) -> f64
    {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
}

impl fmt::Display for Tuple
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

pub fn create_point(x: f64, y: f64, z: f64) -> Tuple
{
    Tuple{x: x, y: y, z: z, w: 1.0}
}

pub fn create_vector(x: f64, y: f64, z: f64) -> Tuple
{
    Tuple{x: x, y: y, z: z, w: 0.0}
}

pub fn create_color(r: f64, g: f64, b: f64) -> Tuple
{
    Tuple{x: r, y: g, z: b, w: 0.0}
}

pub fn create_tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple
{
    Tuple{x: x, y: y, z: z, w: w}
}

pub fn cross_product(a: Tuple, b: Tuple) -> Tuple
{
    create_vector(a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x)
}

pub fn hadamard_product(a: Tuple, b: Tuple) -> Tuple
{
    create_color(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn equal(a: Tuple, b: Tuple) -> bool
{
    fuzzy_equal(a.x, b.x) &&
        fuzzy_equal(a.y, b.y) &&
        fuzzy_equal(a.z, b.z) &&
        fuzzy_equal(a.w, b.w)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_tuples_feature()
    {
        // p.4 Scenario: A tuple with w=1 is a point
        let p1 = create_point(4.3, -4.2, 3.1).get_vec();
        assert_eq!(p1.len(), 4);
        assert!(fuzzy_equal(p1[0], 4.3));
        assert!(fuzzy_equal(p1[1], -4.2));
        assert!(fuzzy_equal(p1[2], 3.1));
        assert!(fuzzy_equal(p1[3], 1.0));

        // p.4 Scenario: A tuple with w=0 is a vector
        let v1 = create_vector(4.3, -4.2, 3.1).get_vec();
        assert_eq!(v1.len(), 4);
        assert!(fuzzy_equal(v1[0], 4.3));
        assert!(fuzzy_equal(v1[1], -4.2));
        assert!(fuzzy_equal(v1[2], 3.1));
        assert!(fuzzy_equal(v1[3], 0.0));

        // p.5 Scenario: Adding two tuples
        let a1 = create_point(3.0, -2.0, 5.0)
            .add(create_vector(-2.0, 3.0, 1.0)).get_vec();
        assert_eq!(a1.len(), 4);
        assert!(fuzzy_equal(a1[0], 1.0));
        assert!(fuzzy_equal(a1[1], 1.0));
        assert!(fuzzy_equal(a1[2], 6.0));
        assert!(fuzzy_equal(a1[3], 1.0));

        // p.5 Scenario: Subtracting two points
        let s1 = create_point(3.0, 2.0, 1.0)
            .sub(create_point(5.0, 6.0, 7.0));
        let s2 = create_vector(-2.0, -4.0, -6.0);
        assert!(equal(s1, s2));

        // p.5 Scenario: Subtracting a vector from a point
        let s3 = create_point(3.0, 2.0, 1.0)
            .sub(create_vector(5.0, 6.0, 7.0));
        let s4 = create_point(-2.0, -4.0, -6.0);
        assert!(equal(s3, s4));

        // p.7 Scenario: Subtracting two vectors
        let s5 = create_vector(3.0, 2.0, 1.0)
            .sub(create_vector(5.0, 6.0, 7.0));
        let s6 = create_vector(-2.0, -4.0, -6.0);
        assert!(equal(s5, s6));

        // p.7 Scenario: Subtracting a vector from the zero vector
        let s7 = create_vector(0.0, 0.0, 0.0)
            .sub(create_vector(1.0, -2.0, 3.0));
        let s8 = create_vector(-1.0, 2.0, -3.0);
        assert!(equal(s7, s8));

        // p.7 Scenario: Negating a tuple
        let n1 = create_tuple(1.0, -2.0, 3.0, -4.0).negate();
        let n2 = create_tuple(-1.0, 2.0, -3.0, 4.0);
        assert!(equal(n1, n2));

        // p.8 Scenario: Multiplying a tuple by a scalar
        let m1 = create_tuple(1.0, -2.0, 3.0, -4.0).multiply(3.5);
        let m2 = create_tuple(3.5, -7.0, 10.5, -14.0);
        assert!(equal(m1, m2));

        // p.8 Scenario: Multiplying a tuple by a fraction
        let m3 = create_tuple(1.0, -2.0, 3.0, -4.0).multiply(0.5);
        let m4 = create_tuple(0.5, -1.0, 1.5, -2.0);
        assert!(equal(m3, m4));

        // p.8 Scenario: Dividing a tuple by a scalar
        let d1 = create_tuple(1.0, -2.0, 3.0, -4.0).divide(2.0);
        let d2 = create_tuple(0.5, -1.0, 1.5, -2.0);
        assert!(equal(d1, d2));

        // p.8 Scenario: Computing the magnitude of vector(1, 0, 0)
        let m1 = create_vector(1.0, 0.0, 0.0).magnitude();
        assert!(fuzzy_equal(m1, 1.0));

        // p.9 Scenario: Computing the magnitude of vector(0, 1, 0)
        let m2 = create_vector(0.0, 1.0, 0.0).magnitude();
        assert!(fuzzy_equal(m2, 1.0));

        // p.9 Scenario: Computing the magnitude of vector(0, 0, 1)
        let m3 = create_vector(0.0, 0.0, 1.0).magnitude();
        assert!(fuzzy_equal(m3, 1.0));

        // p.9 Scenario: Computing the magnitude of vector(1, 2, 3)
        let m4 = create_vector(1.0, 2.0, 3.0).magnitude();
        assert!(fuzzy_equal(m4, 14.0_f64.sqrt()));

        // p.9 Scenario: Computing the magnitude of vector(-1, -2, -3)
        let m5 = create_vector(-1.0, -2.0, -3.0).magnitude();
        assert!(fuzzy_equal(m5, 14.0_f64.sqrt()));

        // p.10 Scenario: Normalizing the vector (4, 0, 0) gives (1, 0, 0)
        let no1 = create_vector(4.0, 0.0, 0.0).normalize();
        let no2 = create_vector(1.0, 0.0, 0.0);
        assert!(equal(no1, no2));

        // p.10 Scenario: Normalizing the vector (1, 2, 3)
        let no3 = create_vector(1.0, 2.0, 3.0).normalize();
        let no4 = create_vector(0.26726, 0.53452, 0.80178);
        assert!(equal(no3, no4));

        // p.10 Scenario: The magnitude of a normalized vector
        let m6 = no3.magnitude();
        assert!(fuzzy_equal(m6, 1.0));

        // p.10 Scenario: The dot product of two tuples
        let dp1 = create_vector(1.0, 2.0, 3.0)
            .dot_product(create_vector(2.0, 3.0, 4.0));
        assert!(fuzzy_equal(dp1, 20.0));

        // p.11 Scenario: The cross product of two vectors
        let cp1 = cross_product(create_vector(1.0, 2.0, 3.0),
            create_vector(2.0, 3.0, 4.0));
        assert!(equal(cp1, create_vector(-1.0, 2.0, -1.0)));
        let cp2 = cross_product(create_vector(2.0, 3.0, 4.0),
            create_vector(1.0, 2.0, 3.0));
        assert!(equal(cp2, create_vector(1.0, -2.0, 1.0)));

        // p.16 Scenario: Colors are (red, green, blue) tuples
        let c1 = create_color(-0.5, 0.4, 1.7).get_vec();
        assert!(fuzzy_equal(c1[0], -0.5));
        assert!(fuzzy_equal(c1[1], 0.4));
        assert!(fuzzy_equal(c1[2], 1.7));

        // p.17 Scenario: Adding colors
        let c2 = create_color(0.9, 0.6, 0.75)
            .add(create_color(0.7, 0.1, 0.25));
        let c3 = create_color(1.6, 0.7, 1.0);
        assert!(equal(c2, c3));

        // p.17 Scenario: Subtracting colors
        let c4 = create_color(0.9, 0.6, 0.75)
            .sub(create_color(0.7, 0.1, 0.25));
        let c5 = create_color(0.2, 0.5, 0.5);
        assert!(equal(c4, c5));

        // p.17 Scenario: Mutiplying a color by a scalar
        let c6 = create_color(0.2, 0.3, 0.4).multiply(2.0);
        let c7 = create_color(0.4, 0.6, 0.8);
        assert!(equal(c6, c7));

        // p.17 Scenario: Mutiplying colors
        let c8 = hadamard_product(create_color(1.0, 0.2, 0.4),
            create_color(0.9, 1.0, 0.1));
        let c9 = create_color(0.9, 0.2, 0.04);
        assert!(equal(c8, c9));
    }
}
