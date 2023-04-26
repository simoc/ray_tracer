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
}
