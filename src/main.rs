mod tuple;
mod canvas;

use crate::tuple::*;
use crate::canvas::*;

fn main()
{
    // Write a black PPM image to stdout until something more
    // useful is possible.
    let c = create_canvas(100, 100);
    print!("{}", c.canvas_to_ppm());
}
