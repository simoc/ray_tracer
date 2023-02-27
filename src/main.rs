mod arithmetic;
mod tuple;
mod canvas;

use crate::tuple::*;
use crate::canvas::*;

fn main()
{
    // Write a black PPM image to stdout until something more
    // useful is possible.
    let c = Canvas::new(100, 100);
    print!("{}", c.to_ppm());
}
