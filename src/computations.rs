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
    pub reflectv: Tuple,
}

impl Computations
{
    pub fn new(t: f64, object: Shape, point: Tuple, eyev: Tuple,
         normalv: Tuple, inside: bool, over_point: Tuple,
         reflectv: Tuple) -> Self
    {
        Computations{t, object, point, eyev, normalv, inside, over_point,
            reflectv}
    }
}
