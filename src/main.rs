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
mod plane;
mod pattern;
mod cube;
mod cylinder;
mod cone;
mod group;
mod triangle;

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
use crate::plane::*;
use crate::pattern::*;
use crate::cube::*;
use crate::cylinder::*;
use crate::cone::*;
use crate::group::*;
use crate::triangle::*;

fn hexagon_corner(id: i32) -> Shape
{
    let mut corner = Shape::new_sphere(id);
    corner.set_transform(Matrix::translation(0.0, 0.0, -1.0)
        .multiply(&Matrix::scaling(0.25, 0.25, 0.25)));
    return corner;
}

fn hexagon_edge(id: i32) -> Shape
{
    let mut edge = Shape::new_cylinder(id, true, 0.0, 1.0);
    edge.set_transform(Matrix::translation(0.0, 0.0, -1.0)
        .multiply(&Matrix::rotation_y(-PI / 6.0))
        .multiply(&Matrix::rotation_z(-PI / 2.0))
        .multiply(&Matrix::scaling(0.25, 1.0, 0.25)));
    return edge;
}

fn hexagon_side(id: i32) -> Shape
{
    let mut side = Shape::new_group(id);
    side.add_child(&mut hexagon_corner(id + 1));
    side.add_child(&mut hexagon_edge(id + 2));
    return side;
}

fn hexagon(id: i32) -> Shape
{
    let mut hex = Shape::new_group(id);
    for n in 0..6
    {
        let mut side = hexagon_side(id + (n + 1) * 10);
        side.set_transform(Matrix::rotation_y(f64::from(n) * PI / 3.0));
        hex.add_child(&mut side);
    }
    let mut material = Material::new();
    material.color = create_color(0.8, 0.1, 0.8);
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.reflective = 0.6;
    hex.set_material(material);
    return hex;
}

fn pyramid(id: i32) -> Shape
{
    let mut pyramid = Shape::new_group(id);
    let mut material = Material::new();
    material.color = create_color(0.1, 0.1, 5.0);
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.shininess = 100.0;
    material.pattern = None;
    material.reflective = 0.9;
    material.transparency = 0.0;

    let size = 0.5;
    let p1 = create_point(-size, 0.0, -size);
    let p2 = create_point(-size, 0.0, size);
    let p3 = create_point(size, 0.0, size);
    let p4 = create_point(size, 0.0, -size);
    let p5 = create_point(0.0, size, 0.0);

    let mut triangle1 = Shape::new_triangle(id + 1, p1, p2, p4);
    triangle1.set_material(material.clone());
    let mut triangle2 = Shape::new_triangle(id + 2, p2, p3, p4);
    triangle2.set_material(material.clone());

    let mut triangle3 = Shape::new_triangle(id + 3, p1, p2, p5);
    triangle3.set_material(material.clone());
    let mut triangle4 = Shape::new_triangle(id + 4, p2, p3, p5);
    triangle4.set_material(material.clone());
    let mut triangle5 = Shape::new_triangle(id + 5, p3, p4, p5);
    triangle5.set_material(material.clone());
    let mut triangle6 = Shape::new_triangle(id + 6, p4, p1, p5);
    triangle6.set_material(material.clone());

    pyramid.add_child(&mut triangle1);
    pyramid.add_child(&mut triangle2);
    pyramid.add_child(&mut triangle3);
    pyramid.add_child(&mut triangle4);
    pyramid.add_child(&mut triangle5);
    pyramid.add_child(&mut triangle6);
    pyramid.set_material(material.clone());

    return pyramid;
}

fn main()
{
    // p.107 Chapter 7, Putting It Together

    // 1. The floor is a plane with a matte texture and stripe pattern.
    let mut floor = Shape::new_plane(1);
    let mut floor_material = Material::new();
    floor_material.color = create_color(1.0, 0.9, 0.9);
    floor_material.specular = 0.0;
    let mut floor_pattern = Pattern::new_stripe_pattern(create_color(0.5, 0.5, 0.0),
        create_color(0.8, 0.8, 0.0));
    floor_pattern.set_pattern_transform(Matrix::scaling(1.0, 1.0, 1.0));
    floor_material.pattern = Some(floor_pattern);
    floor.set_material(floor_material);

    // 4. The large sphere in the middle is a unit sphere, translated upward
    // slightly and colored green.
    let mut middle_sphere = Shape::new_sphere(4);
    middle_sphere.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
    let mut middle_sphere_material = Material::new();
    middle_sphere_material.color = create_color(0.1, 1.0, 0.5);
    middle_sphere_material.diffuse = 0.7;
    middle_sphere_material.specular = 0.3;
    let mut middle_sphere_pattern = Pattern::new_checker_pattern(create_color(0.0, 0.5, 0.5),
        create_color(0.0, 0.8, 0.8));
    middle_sphere_pattern.set_pattern_transform(Matrix::scaling(0.5, 0.5, 0.5));
    middle_sphere_material.pattern = Some(middle_sphere_pattern);
    middle_sphere.set_material(middle_sphere_material);

    // 5. The smaller green sphere on the right is scalled in half.
    let mut right_sphere = Shape::new_sphere(5);
    let right_sphere_translation = Matrix::translation(1.5, 0.5, -0.5);
    let right_sphere_scaling = Matrix::scaling(0.5, 0.5, 0.5);
    right_sphere.set_transform(right_sphere_translation.multiply(&right_sphere_scaling));
    let mut right_sphere_material = Material::new();
    right_sphere_material.color = create_color(0.5, 1.0, 0.1);
    right_sphere_material.diffuse = 0.7;
    right_sphere_material.specular = 0.3;
    right_sphere_material.transparency = 0.9;
    right_sphere_material.reflective = 0.9;
    right_sphere_material.diffuse = 0.2;
    right_sphere.set_material(right_sphere_material);

    // 6. The smallest sphere is scaled by a third, before being translated.
    let mut left_sphere = Shape::new_sphere(6);
    let left_sphere_translation = Matrix::translation(-1.5, 0.33, -0.75);
    let left_sphere_scaling = Matrix::scaling(0.33, 0.33, 0.33);
    left_sphere.set_transform(left_sphere_translation.multiply(&left_sphere_scaling));
    let mut left_sphere_material = Material::new();
    left_sphere_material.color = create_color(1.0, 0.8, 0.1);
    left_sphere_material.diffuse = 0.7;
    left_sphere_material.specular = 0.3;
    left_sphere.set_material(left_sphere_material);

    // 7. A cube with high z value, far in the background (to test that shape too)
    let mut cube = Shape::new_cube(7);
    let cube_translation = Matrix::translation(4.0, 1.0, 9.0);
    let cube_scaling = Matrix::scaling(0.66, 0.66, 0.66);
    let cube_rotation = Matrix::rotation_x(PI / 4.0);
    cube.set_transform(cube_translation.multiply(&cube_rotation.multiply(&cube_scaling)));
    let mut cube_material = Material::new();
    cube_material.color = create_color(0.8, 0.1, 0.1);
    cube_material.diffuse = 0.5;
    cube.set_material(cube_material);

    // 8. A cylinder with high z value, far in the background (to test that shape too)
    let mut cylinder = Shape::new_cylinder(8, true, 0.0, 2.0);
    let cylinder_translation = Matrix::translation(-6.0, 0.707, 9.0);
    let cylinder_rotation = Matrix::rotation_x(-PI / 4.0);
    cylinder.set_transform(cylinder_translation.multiply(&cylinder_rotation));
    let mut cylinder_material = Material::new();
    cylinder_material.color = create_color(0.6, 0.6, 0.6);
    cylinder_material.ambient = 0.2;
    cylinder_material.reflective = 0.7;
    cylinder_material.diffuse = 0.3;
    cylinder_material.shininess = 100.0;
    cylinder.set_material(cylinder_material);

    // A group of objects making up a hexagon
    let mut hex1 = hexagon(9);
    hex1.set_transform(Matrix::translation(5.0, 1.0, 4.0).multiply(&Matrix::rotation_x(-PI / 4.0)));
    let mut hex2 = hexagon(99);
    hex2.set_transform(Matrix::translation(-5.4, 1.0, 2.0).multiply(&Matrix::rotation_x(-PI / 4.0)));

    let mut pyramid1 = pyramid(333);
    pyramid1.set_transform(Matrix::translation(3.0, 0.0, -1.0));
    let mut pyramid2 = pyramid(444);
    pyramid2.set_transform(Matrix::translation(1.0, 0.0, -2.0));
    let mut pyramid3 = pyramid(555);
    pyramid3.set_transform(Matrix::translation(-1.0, 0.0, -2.0));
    let mut pyramid4 = pyramid(666);
    pyramid4.set_transform(Matrix::translation(-3.0, 0.0, -1.0));

    // The light source is white, shining from above and to the left:
    let mut world = World::default_world();
    world.light = PointLight::new(create_point(-10.0, 10.0, -10.0), create_color(1.0, 1.0, 1.0));
    world.objects = vec![floor,
        middle_sphere, right_sphere, left_sphere,
        cube, cylinder,
        hex1, hex2,
        pyramid1, pyramid2, pyramid3, pyramid4];

    // And the camera is configured like so:
    let mut camera = Camera::new(100, 50, PI / 2.0);
    camera.transform = Matrix::view_transform(create_point(0.0, 1.5, -5.0),
        create_point(0.0, 1.0, 0.0),
        create_vector(0.0, 1.0, 0.0));

    // render the result to a canvas.
    let canvas = camera.render(world);
    print!("{}", canvas.to_ppm());
}
