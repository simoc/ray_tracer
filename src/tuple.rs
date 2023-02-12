#[derive(Copy, Clone)]
pub struct Tuple
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple
{
    pub fn get_vec(self) -> Vec<f64>
    {
        vec![self.x, self.y, self.z, self.w]
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

pub fn create_tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple
{
    Tuple{x: x, y: y, z: z, w: w}
}

pub fn add(a: Tuple, b: Tuple) -> Tuple
{
    Tuple{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w}
}

pub fn sub(a: Tuple, b: Tuple) -> Tuple
{
    Tuple{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z, w: a.w - b.w}
}

pub fn negate(a: Tuple) -> Tuple
{
    Tuple{x: -a.x, y: -a.y, z: -a.z, w: -a.w}
}

pub fn multiply(a: Tuple, scalar: f64) -> Tuple
{
    Tuple{x: a.x * scalar, y: a.y * scalar, z: a.z * scalar, w: a.w * scalar}
}

pub fn divide(a: Tuple, scalar: f64) -> Tuple
{
    Tuple{x: a.x / scalar, y: a.y / scalar, z: a.z / scalar, w: a.w / scalar}
}

pub fn magnitude(v: Tuple) -> f64
{
    let n = (v.x * v.x) + (v.y * v.y) + (v.z * v.z) + (v.w * v.w);
    n.sqrt()
}

pub fn normalize(v: Tuple) -> Tuple
{
	let m = magnitude(v);
    Tuple{x: v.x / m, y: v.y / m, z: v.z / m, w: v.w / m}
}

pub fn dot_product(a: Tuple, b: Tuple) -> f64
{
	a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn fuzzy_equal(a: f64, b: f64) -> bool
{
	let epsilon = 0.00001;
	let diff = a - b;
	diff.abs() < epsilon
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
        let a1 = add(create_point(3.0, -2.0, 5.0),
			create_vector(-2.0, 3.0, 1.0)).get_vec();
        assert_eq!(a1.len(), 4);
        assert!(fuzzy_equal(a1[0], 1.0));
        assert!(fuzzy_equal(a1[1], 1.0));
        assert!(fuzzy_equal(a1[2], 6.0));
        assert!(fuzzy_equal(a1[3], 1.0));

		// p.5 Scenario: Subtracting two points
        let s1 = sub(create_point(3.0, 2.0, 1.0),
			create_point(5.0, 6.0, 7.0));
		let s2 = create_vector(-2.0, -4.0, -6.0);
        assert!(equal(s1, s2));

		// p.5 Scenario: Subtracting a vector from a point
        let s3 = sub(create_point(3.0, 2.0, 1.0),
			create_vector(5.0, 6.0, 7.0));
		let s4 = create_point(-2.0, -4.0, -6.0);
        assert!(equal(s3, s4));

		// p.7 Scenario: Subtracting two vectors
        let s5 = sub(create_vector(3.0, 2.0, 1.0),
			create_vector(5.0, 6.0, 7.0));
		let s6 = create_vector(-2.0, -4.0, -6.0);
        assert!(equal(s5, s6));

		// p.7 Scenario: Subtracting a vector from the zero vector
        let s7 = sub(create_vector(0.0, 0.0, 0.0),
			create_vector(1.0, -2.0, 3.0));
		let s8 = create_vector(-1.0, 2.0, -3.0);
        assert!(equal(s7, s8));

		// p.7 Scenario: Negating a tuple
        let n1 = negate(create_tuple(1.0, -2.0, 3.0, -4.0));
		let n2 = create_tuple(-1.0, 2.0, -3.0, 4.0);
        assert!(equal(n1, n2));

        // p.8 Scenario: Multiplying a tuple by a scalar
        let m1 = multiply(create_tuple(1.0, -2.0, 3.0, -4.0), 3.5);
        let m2 = create_tuple(3.5, -7.0, 10.5, -14.0);
        assert!(equal(m1, m2));

        // p.8 Scenario: Multiplying a tuple by a fraction
        let m3 = multiply(create_tuple(1.0, -2.0, 3.0, -4.0), 0.5);
        let m4 = create_tuple(0.5, -1.0, 1.5, -2.0);
        assert!(equal(m3, m4));

        // p.8 Scenario: Dividing a tuple by a scalar
        let d1 = divide(create_tuple(1.0, -2.0, 3.0, -4.0), 2.0);
        let d2 = create_tuple(0.5, -1.0, 1.5, -2.0);
        assert!(equal(d1, d2));

        // p.8 Scenario: Computing the magnitude of vector(1, 0, 0)
        let m1 = magnitude(create_vector(1.0, 0.0, 0.0));
        assert!(fuzzy_equal(m1, 1.0));

        // p.9 Scenario: Computing the magnitude of vector(0, 1, 0)
        let m2 = magnitude(create_vector(0.0, 1.0, 0.0));
        assert!(fuzzy_equal(m2, 1.0));

        // p.9 Scenario: Computing the magnitude of vector(0, 0, 1)
        let m3 = magnitude(create_vector(0.0, 0.0, 1.0));
        assert!(fuzzy_equal(m3, 1.0));

        // p.9 Scenario: Computing the magnitude of vector(1, 2, 3)
        let m4 = magnitude(create_vector(1.0, 2.0, 3.0));
        assert!(fuzzy_equal(m4, 14.0_f64.sqrt()));

        // p.9 Scenario: Computing the magnitude of vector(-1, -2, -3)
        let m5 = magnitude(create_vector(-1.0, -2.0, -3.0));
        assert!(fuzzy_equal(m5, 14.0_f64.sqrt()));

        // p.10 Scenario: Normalizing the vector (4, 0, 0) gives (1, 0, 0)
        let no1 = normalize(create_vector(4.0, 0.0, 0.0));
		let no2 = create_vector(1.0, 0.0, 0.0);
        assert!(equal(no1, no2));

        // p.10 Scenario: Normalizing the vector (1, 2, 3)
        let no3 = normalize(create_vector(1.0, 2.0, 3.0));
		let no4 = create_vector(0.26726, 0.53452, 0.80178);
        assert!(equal(no3, no4));

        // p.10 Scenario: The magnitude of a normalized vector
		let m6 = magnitude(no3);
        assert!(fuzzy_equal(m6, 1.0));

        // p.10 Scenario: The dot product of two tuples
		let dp1 = dot_product(create_vector(1.0, 2.0, 3.0),
			create_vector(2.0, 3.0, 4.0));
        assert!(fuzzy_equal(dp1, 20.0));
    }
}
