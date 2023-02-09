pub struct Tuple
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple
{
    pub fn get_vector(self) -> Vec<f64>
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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_tuples_feature()
    {
        let v = create_point(4.3, -4.2, 3.1).get_vector();
        assert_eq!(v.len(), 4);
    }
}
