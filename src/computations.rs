use crate::sphere::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct Computations
{
    pub t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl Computations
{
    pub fn new(t: f64, object: Sphere, point: Tuple, eyev: Tuple,
         normalv: Tuple, inside: bool, over_point: Tuple) -> Self
    {
        Computations{t, object, point, eyev, normalv, inside, over_point}
    }
}
