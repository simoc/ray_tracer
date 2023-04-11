use crate::arithmetic::*;
use crate::computations::*;
use crate::intersections::*;
use crate::material::*;
use crate::matrix::*;
use crate::pointlight::*;
use crate::ray::*;
use crate::sphere::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct World
{
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World
{
    pub fn default_world() -> Self
    {
        let point = create_point(-10.0, 10.0, -10.0);
        let intensity = create_color(1.0, 1.0, 1.0);
        let light = PointLight::new(point, intensity);
        let mut sphere1 = Sphere::new(1);
        let mut material = Material::new();
        material.color = create_color(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        sphere1.set_material(material);

        let mut sphere2 = Sphere::new(2);
        sphere2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        World{light: light, objects: vec![sphere1, sphere2]}
    }

    pub fn intersect_world(&self, ray: Ray) -> Intersections
    {
        let mut intersections = Vec::new();
        for object in &self.objects
        {
            let xs = object.intersect(ray);
            for t in xs
            {
                intersections.push(Intersection::new(t, object.clone()));
            }
        }
        Intersections::new(intersections)
    }

    pub fn shade_hit(&self, comps: Computations) -> Tuple
    {
        comps.object.get_material().lighting(self.light, comps.point,
            comps.eyev, comps.normalv, false)
    }

    pub fn color_at(&self, ray: Ray) -> Tuple
    {
        let intersections = self.intersect_world(ray);
        match intersections.hit()
        {
            Some(intersection) =>
            {
                let comps = intersection.prepare_computation(ray);
                self.shade_hit(comps)
            },
            None => create_color(0.0, 0.0, 0.0),
        }
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
        assert!(world1.objects.contains(&Sphere::new(1)));
        assert!(world1.objects.contains(&Sphere::new(2)));

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
        let shape3 = Sphere::new(3);
        let intersection3 = Intersection::new(4.0, shape3.clone());
        let comps3 = intersection3.prepare_computation(ray3);
        assert!(fuzzy_equal(comps3.t, intersection3.t));
        assert_eq!(comps3.object, intersection3.object);
        assert_eq!(comps3.point, create_point(0.0, 0.0, -1.0));
        assert_eq!(comps3.eyev, create_vector(0.0, 0.0, -1.0));
        assert_eq!(comps3.normalv, create_vector(0.0, 0.0, -1.0));

        // p.94 Scenario: The hit, when an intersection occurs on the outside
        let ray4 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let shape4 = Sphere::new(4);
        let intersection4 = Intersection::new(4.0, shape4.clone());
        let comps4 = intersection4.prepare_computation(ray4);
        assert!(comps4.inside == false);

        // p.95 Scenario: The hit, when an intersection occurs on the inside
        let ray5 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let shape5 = Sphere::new(5);
        let intersection5 = Intersection::new(1.0, shape5.clone());
        let comps5 = intersection5.prepare_computation(ray5);
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
        let comps6 = intersection6.prepare_computation(ray6);
        let color6 = world6.shade_hit(comps6);
        assert_eq!(color6, create_color(0.38066, 0.47583, 0.2855));

        // p.95 Scenario: Shading an intersection
        let mut world7 = World::default_world();
        world7.light = PointLight::new(create_point(0.0, 0.25, 0.0), create_color(1.0, 1.0, 1.0));
        let ray7 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let shape7 = world7.objects[1].clone();
        let intersection7 = Intersection::new(0.5, shape7.clone());
        let comps7 = intersection7.prepare_computation(ray7);
        let color7 = world7.shade_hit(comps7);
        assert_eq!(color7, create_color(0.90498, 0.90498, 0.90498));

        // p.96 Scenario: Color when a ray misses
        let world8 = World::default_world();
        let ray8 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 1.0, 0.0));
        let color8 = world8.color_at(ray8);
        assert_eq!(color8, create_color(0.0, 0.0, 0.0));

        // p.96 Scenario: Color when a ray hits
        let world9 = World::default_world();
        let ray9 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let color9 = world9.color_at(ray9);
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
        inner10.set_material(inner_material10);
        world10.objects[1] = inner10;
        let ray10 = Ray::new(create_point(0.0, 0.0, 0.75), create_vector(0.0, 0.0, -1.0));
        let color10 = world10.color_at(ray10);
        assert_eq!(color10, inner_material10.color);
    }
}
