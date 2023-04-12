mod arithmetic;
mod tuple;
mod canvas;
mod sphere;
mod matrix;
mod ray;
mod intersections;
mod material;
mod pointlight;
mod computations;
mod world;
mod camera;
mod shape;

use std::f64::consts::PI;
use crate::tuple::*;
use crate::canvas::*;
use crate::sphere::*;
use crate::ray::*;
use crate::intersections::*;
use crate::material::*;
use crate::pointlight::*;
use crate::computations::*;
use crate::world::*;
use crate::camera::*;
use crate::matrix::*;
use crate::shape::*;

fn main()
{
    // p.107 Chapter 7, Putting It Together

    // 1. The floor is an extremely flattened sphere with a matte texture.
    let mut floor = Sphere::new(1);
    floor.set_transform(Matrix::scaling(10.0, 0.1, 10.0));
    let mut floor_material = Material::new();
    floor_material.color = create_color(1.0, 0.9, 0.9);
    floor_material.specular = 0.0;
    floor.set_material(floor_material);

    // 2. The wall on the left has the same scale and color as the floor,
    // but is also rotated and translated into place.
    let mut left_wall = Sphere::new(2);
    let left_wall_translation = Matrix::translation(0.0, 0.0, 5.0);
    let left_wall_rotation_y = Matrix::rotation_y(-PI / 4.0);
    let left_wall_rotation_x = Matrix::rotation_x(PI / 2.0);
    let left_wall_scaling = Matrix::scaling(10.0, 0.01, 10.0);
    left_wall.set_transform(left_wall_translation.multiply(&left_wall_rotation_y).
        multiply(&left_wall_rotation_x).multiply(&left_wall_scaling));
    left_wall.set_material(floor_material);

    // 3. The wall on the right is identical to the left wall, but is rotated
    // the opposite direction in y.
    let mut right_wall = Sphere::new(3);
    let right_wall_translation = left_wall_translation;
    let right_wall_rotation_y = Matrix::rotation_y(PI / 4.0);
    let right_wall_rotation_x = left_wall_rotation_x;
    let right_wall_scaling = left_wall_scaling;
    right_wall.set_transform(right_wall_translation.multiply(&right_wall_rotation_y).
        multiply(&right_wall_rotation_x).multiply(&right_wall_scaling));
    right_wall.set_material(floor_material);

    // 4. The large sphere in the middle is a unit sphere, translated upward
    // slightly and colored green.
    let mut middle_sphere = Sphere::new(4);
    middle_sphere.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
    let mut middle_sphere_material = Material::new();
    middle_sphere_material.color = create_color(0.1, 1.0, 0.5);
    middle_sphere_material.diffuse = 0.7;
    middle_sphere_material.specular = 0.3;
    middle_sphere.set_material(middle_sphere_material);

    // 5. The smaller green sphere on the right is scalled in half.
    let mut right_sphere = Sphere::new(5);
    let right_sphere_translation = Matrix::translation(1.5, 0.5, -0.5);
    let right_sphere_scaling = Matrix::scaling(0.5, 0.5, 0.5);
    right_sphere.set_transform(right_sphere_translation.multiply(&right_sphere_scaling));
    let mut right_sphere_material = Material::new();
    right_sphere_material.color = create_color(0.5, 1.0, 0.1);
    right_sphere_material.diffuse = 0.7;
    right_sphere_material.specular = 0.3;
    right_sphere.set_material(right_sphere_material);

    // 6. The smallest sphere is scalled by a third, before being translated.
    let mut left_sphere = Sphere::new(6);
    let left_sphere_translation = Matrix::translation(-1.5, 0.33, -0.75);
    let left_sphere_scaling = Matrix::scaling(0.33, 0.33, 0.33);
    left_sphere.set_transform(left_sphere_translation.multiply(&left_sphere_scaling));
    let mut left_sphere_material = Material::new();
    left_sphere_material.color = create_color(1.0, 0.8, 0.1);
    left_sphere_material.diffuse = 0.7;
    left_sphere_material.specular = 0.3;
    left_sphere.set_material(left_sphere_material);

    // The light source is white, shining from above and to the left:
    let mut world = World::default_world();
    world.light = PointLight::new(create_point(-10.0, 10.0, -10.0), create_color(1.0, 1.0, 1.0));
    world.objects = vec![floor, left_wall, right_wall,
        middle_sphere, right_sphere, left_sphere];

    // And the camera is configured like so:
    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform = Matrix::view_transform(create_point(0.0, 1.5, -5.0),
        create_point(0.0, 1.0, 0.0),
        create_vector(0.0, 1.0, 0.0));

    // render the result to a canvas.
    let canvas = camera.render(world);
    print!("{}", canvas.to_ppm());
}
