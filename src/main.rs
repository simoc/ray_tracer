mod arithmetic;
mod tuple;
mod canvas;
mod sphere;
mod matrix;
mod ray;
mod intersections;

use crate::tuple::*;
use crate::canvas::*;
use crate::sphere::*;
use crate::ray::*;
use crate::intersections::*;

fn main()
{
    // Start the ray at z = -5
    let ray_origin = create_point(0.0, 0.0, -5.0);

    // Put the wall at z = 10
    let wall_z: f64 = 10.0;

    let wall_size: f64 = 7.0;

    let canvas_pixels: u16 = 100;

    let pixel_size = wall_size / f64::from(canvas_pixels);

    let half_wall_size = wall_size / 2.0;

    let mut c = Canvas::new(usize::from(canvas_pixels), usize::from(canvas_pixels));
    let red_color = create_color(1.0, 0.0, 0.0);

    let shape = Sphere::new(1);

    // for each row of ipxels in the canvas
    for y in 0..canvas_pixels - 1
    {
        // for each pixel in the row
        let world_y: f64 = half_wall_size - pixel_size * f64::from(y);
        for x in 0..canvas_pixels - 1
        {
            // compute the world x coordinate (left = -half_wall_size, right = half_wall_size)
            let world_x: f64 = -half_wall_size + pixel_size * f64::from(x);

            // describe the point on the wall that the ray will target
            let position = create_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, position.sub(ray_origin));
            let xs = shape.intersect(r);
            if xs.len() == 2
            {
                let intersection1 = Intersection::new(xs[0], shape.clone());
                let intersection2 = Intersection::new(xs[1], shape.clone());
                let is = Intersections::new(vec![intersection1, intersection2]);
                let h = is.hit();
                match h
                {
                    Some(h) => c.write_pixel(usize::from(x), usize::from(y), red_color),
                    _ => (),
                }
            }
        }
    }
    print!("{}", c.to_ppm());
}
