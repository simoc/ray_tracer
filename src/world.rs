use crate::arithmetic::*;
use crate::computations::*;
use crate::intersections::*;
use crate::material::*;
use crate::matrix::*;
use crate::pointlight::*;
use crate::ray::*;
use crate::shape::*;
use crate::sphere::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct World
{
    pub light: PointLight,
    pub objects: Vec<Shape>,
}

impl World
{
    // maximum number of times to reflect rays, to avoid infinite recursion
    pub const REFLECTION_RECURSION: i32 = 4;

    pub fn default_world() -> Self
    {
        let point = create_point(-10.0, 10.0, -10.0);
        let intensity = create_color(1.0, 1.0, 1.0);
        let light = PointLight::new(point, intensity);
        let mut sphere1 = Shape::new_sphere(1);
        let mut material = Material::new();
        material.color = create_color(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        sphere1.set_material(material);

        let mut sphere2 = Shape::new_sphere(2);
        sphere2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        World{light: light, objects: vec![sphere1, sphere2]}
    }

    pub fn intersect_world(&self, ray: Ray) -> Intersections
    {
        let mut intersections = Vec::new();
        for object in &self.objects
        {
            let xs = object.clone().intersect(ray);
            for t in xs
            {
                intersections.push(Intersection::new(t, object.clone()));
            }
        }
        Intersections::new(intersections)
    }

    pub fn shade_hit(&self, comps: Computations, remaining: i32) -> Tuple
    {
        let comps2 = comps.clone();
        let shadowed = self.is_shadowed(comps.over_point);
        let surface = comps.object.get_material().lighting(comps.object,
            self.light, comps.point,
            comps.eyev, comps.normalv, shadowed);
        let reflected = self.reflected_color(comps2, remaining);
        surface.add(reflected)
    }

    pub fn color_at(&self, ray: Ray, remaining: i32) -> Tuple
    {
        let intersections = self.intersect_world(ray);
        match intersections.hit()
        {
            Some(intersection) =>
            {
                let comps = intersection.prepare_computations(ray, intersections);
                self.shade_hit(comps, remaining)
            },
            None => create_color(0.0, 0.0, 0.0),
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool
    {
        let v = self.light.position.sub(point);
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);
        let intersections = self.intersect_world(r);
        let h = intersections.hit();
        match h
        {
            Some(intersection) => intersection.t < distance,
            None => false
        }
    }

    pub fn reflected_color(&self, comps: Computations, remaining: i32) -> Tuple
    {
        if remaining <= 0
        {
            return create_color(0.0, 0.0, 0.0);
        }
        if fuzzy_equal(comps.object.get_material().reflective, 0.0)
        {
            return create_color(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(reflect_ray, remaining - 1);

        color.multiply(comps.object.get_material().reflective)
    }

    pub fn refracted_color(&self, comps: Computations, remaining: i32) -> Tuple
    {
        if remaining <= 0
        {
            return create_color(0.0, 0.0, 0.0);
        }
        if fuzzy_equal(comps.object.get_material().transparency, 0.0)
        {
            return create_color(0.0, 0.0, 0.0);
        }

        // Find the ratio of first index of refraction to the second.
        // (Yup, this is inverted from the definition of Snell's Law.)
        let n_ratio = comps.n1 / comps.n2;

        // cos(theta_i) is the same as the dot product of the two vectors
        let cos_i = comps.eyev.dot_product(comps.normalv);

        // Find sin(theta_t)^2 via trigonometric identity
        let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos_i * cos_i));

        // Check for total internal reflection
        if sin2_t > 1.0
        {
            return create_color(0.0, 0.0, 0.0);
        }

        return create_color(1.0, 1.0, 1.0);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_world_feature()
    {
        // p.92 Scenario: The default world
        let world1 = World::default_world();
        assert_eq!(world1.light.position, create_point(-10.0, 10.0, -10.0));
        assert_eq!(world1.light.intensity, create_color(1.0, 1.0, 1.0));
        assert!(world1.objects.contains(&Shape::new_sphere(1)));
        assert!(world1.objects.contains(&Shape::new_sphere(2)));

        // p.92 Scenario: Intersect a world with a ray
        let world2 = World::default_world();
        let ray2 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let xs = world2.intersect_world(ray2);
        assert_eq!(xs.count(), 4);
        assert!(fuzzy_equal(xs.get_intersection(0).t, 4.0));
        assert!(fuzzy_equal(xs.get_intersection(1).t, 4.5));
        assert!(fuzzy_equal(xs.get_intersection(2).t, 5.5));
        assert!(fuzzy_equal(xs.get_intersection(3).t, 6.0));

        // p.93 Scenario: Precomputing the state of an intersection
        let ray3 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let shape3 = Shape::new_sphere(3);
        let intersection3 = Intersection::new(4.0, shape3.clone());
        let comps3 = intersection3.prepare_computations(ray3, Intersections::new(vec![intersection3.clone()]));
        assert!(fuzzy_equal(comps3.t, intersection3.t));
        assert_eq!(comps3.object, intersection3.object);
        assert_eq!(comps3.point, create_point(0.0, 0.0, -1.0));
        assert_eq!(comps3.eyev, create_vector(0.0, 0.0, -1.0));
        assert_eq!(comps3.normalv, create_vector(0.0, 0.0, -1.0));

        // p.94 Scenario: The hit, when an intersection occurs on the outside
        let ray4 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let shape4 = Shape::new_sphere(4);
        let intersection4 = Intersection::new(4.0, shape4.clone());
        let comps4 = intersection4.prepare_computations(ray4, Intersections::new(vec![intersection4.clone()]));
        assert!(comps4.inside == false);

        // p.95 Scenario: The hit, when an intersection occurs on the inside
        let ray5 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let shape5 = Shape::new_sphere(5);
        let intersection5 = Intersection::new(1.0, shape5.clone());
        let comps5 = intersection5.prepare_computations(ray5, Intersections::new(vec![intersection5.clone()]));
        assert_eq!(comps5.point, create_point(0.0, 0.0, 1.0));
        assert_eq!(comps5.eyev, create_vector(0.0, 0.0, -1.0));
        assert!(comps5.inside);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps5.normalv, create_vector(0.0, 0.0, -1.0));

        // p.95 Scenario: Shading an intersection
        let world6 = World::default_world();
        let ray6 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let shape6 = world6.objects[0].clone();
        let intersection6 = Intersection::new(4.0, shape6.clone());
        let comps6 = intersection6.prepare_computations(ray6, Intersections::new(vec![intersection6.clone()]));
        let color6 = world6.shade_hit(comps6, World::REFLECTION_RECURSION);
        assert_eq!(color6, create_color(0.38066, 0.47583, 0.2855));

        // p.95 Scenario: Shading an intersection
        let mut world7 = World::default_world();
        world7.light = PointLight::new(create_point(0.0, 0.25, 0.0), create_color(1.0, 1.0, 1.0));
        let ray7 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let shape7 = world7.objects[1].clone();
        let intersection7 = Intersection::new(0.5, shape7.clone());
        let comps7 = intersection7.prepare_computations(ray7, Intersections::new(vec![intersection7.clone()]));
        let color7 = world7.shade_hit(comps7, World::REFLECTION_RECURSION);
        assert_eq!(color7, create_color(0.90498, 0.90498, 0.90498));

        // p.96 Scenario: Color when a ray misses
        let world8 = World::default_world();
        let ray8 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 1.0, 0.0));
        let color8 = world8.color_at(ray8, World::REFLECTION_RECURSION);
        assert_eq!(color8, create_color(0.0, 0.0, 0.0));

        // p.96 Scenario: Color when a ray hits
        let world9 = World::default_world();
        let ray9 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let color9 = world9.color_at(ray9, World::REFLECTION_RECURSION);
        assert_eq!(color9, create_color(0.38066, 0.47583, 0.2855));

        // p.97 Scenario: The color with an intersection behind the ray
        let mut world10 = World::default_world();
        let mut outer10 = world10.objects[0].clone();
        let mut outer_material10 = outer10.get_material();
        outer_material10.ambient = 1.0;
        outer10.set_material(outer_material10);
        world10.objects[0] = outer10;
        let mut inner10 = world10.objects[1].clone();
        let mut inner_material10 = inner10.get_material();
        inner_material10.ambient = 1.0;
        inner10.set_material(inner_material10.clone());
        world10.objects[1] = inner10;
        let ray10 = Ray::new(create_point(0.0, 0.0, 0.75), create_vector(0.0, 0.0, -1.0));
        let color10 = world10.color_at(ray10, World::REFLECTION_RECURSION);
        assert_eq!(color10, inner_material10.color);
    }

    #[test]
    fn test_world_shadow_feature()
    {
        // p.111 Scenario: There is no shadow when nothing collinear with point and light
        let world1 = World::default_world();
        let point1 = create_point(0.0, 10.0, 0.0);
        assert!(world1.is_shadowed(point1) == false);

        // p.112 Scenario: The shadow when an object is between the point and light
        let world2 = World::default_world();
        let point2 = create_point(10.0, -10.0, 10.0);
        assert!(world2.is_shadowed(point2));

        // p.112 Scenario: There is no shadow when object is behind the light
        let world3 = World::default_world();
        let point3 = create_point(-20.0, 20.0, -20.0);
        assert!(world3.is_shadowed(point3) == false);

        // p.112 Scenario: There is no shadow when object is behind the point
        let world4 = World::default_world();
        let point4 = create_point(-2.0, 2.0, -2.0);
        assert!(world4.is_shadowed(point4) == false);

        // p.114 Scenario: shade_hit() is given an intersection in shadow
        let mut world5 = World::default_world();
        world5.light = PointLight::new(create_point(0.0, 0.0, -10.0), create_color(1.0, 1.0, 1.0));
        let sphere1 = Shape::new_sphere(1);
        let mut sphere2 = Shape::new_sphere(2);
        sphere2.set_transform(Matrix::translation(10.0, 0.0, 0.0));
        world5.objects = vec![sphere1.clone(), sphere2.clone()];
        let ray5 = Ray::new(create_point(0.0, 0.0, 5.0), create_vector(0.0, 0.0, 1.0));
        let intersection5 = Intersection::new(4.0, sphere2);
        let comps5 = intersection5.prepare_computations(ray5, Intersections::new(vec![intersection5.clone()]));
        let color5 = world5.shade_hit(comps5, World::REFLECTION_RECURSION);
        assert_eq!(color5, create_color(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_world_reflection_feature()
    {
        // p.146 Scenario: color_at() with mutually reflective surfaces
        let mut world1 = World::default_world();
        world1.light = PointLight::new(create_point(0.0, 0.0, 0.0),
            create_color(1.0, 1.0, 1.0));
        let mut lower = Shape::new_plane(1);
        let mut lower_material = lower.get_material();
        lower_material.reflective = 1.0;
        lower.set_material(lower_material);
        lower.set_transform(Matrix::translation(0.0, -1.0, 0.0));
        let mut upper = Shape::new_plane(2);
        let mut upper_material = upper.get_material();
        upper_material.reflective = 1.0;
        upper.set_material(upper_material);
        upper.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        world1.objects = vec![lower, upper];
        let ray1 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 1.0, 0.0));
        // should terminate successfully
        world1.color_at(ray1, World::REFLECTION_RECURSION);
    }

    #[test]
    fn test_world_refraction_feature()
    {
        // p.155 Scenario: The refracted color with an opaque surface
        let mut world1 = World::default_world();
        let shape1 = world1.objects[0].clone();
        let r1 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let i11 = Intersection::new(4.0, shape1.clone());
        let i12 = Intersection::new(6.0, shape1.clone());
        let xs1 = Intersections::new(vec![i11.clone(), i12.clone()]);
        let comps1 = i11.prepare_computations(r1, xs1);
        let color1 = world1.refracted_color(comps1.clone(), 5);
        assert_eq!(color1, create_color(0.0, 0.0, 0.0));

        // p.156 Scenario: The refracted color at the maximum recursive depth
        let color2 = world1.refracted_color(comps1, 0);
        assert_eq!(color2, create_color(0.0, 0.0, 0.0));

        // p.157 Scenario: The refracted color under total internal reflection
        let mut world3 = World::default_world();
        let shape3 = world3.objects[0].clone();
        let mut material3 = shape3.get_material();
        material3.transparency = 1.0;
        material3.refractive_index = 1.5;
        let sqrt2 = 2.0_f64.sqrt();
        let r3 = Ray::new(create_point(0.0, 0.0, sqrt2 / 2.0),
            create_vector(0.0, 1.0, 0.0));
        let i31 = Intersection::new(-sqrt2 / 2.0, shape3.clone());
        let i32 = Intersection::new(sqrt2 / 2.0, shape3.clone());
        let xs3 = Intersections::new(vec![i31.clone(), i32.clone()]);
        let comps3 = i32.prepare_computations(r3, xs3);
        let color3 = world3.refracted_color(comps3.clone(), 5);
        assert_eq!(color3, create_color(0.0, 0.0, 0.0));
    }
}
