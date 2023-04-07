use crate::arithmetic::*;
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
        let world3 = World::default_world();
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
        let world4 = World::default_world();
        let ray4 = Ray::new(create_point(0.0, 0.0, -5.0), create_vector(0.0, 0.0, 1.0));
        let shape4 = Sphere::new(4);
        let intersection4 = Intersection::new(4.0, shape4.clone());
        let comps4 = intersection4.prepare_computation(ray4);
        assert!(comps4.inside == false);

        // p.95 Scenario: The hit, when an intersection occurs on the inside
        let world5 = World::default_world();
        let ray5 = Ray::new(create_point(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 1.0));
        let shape5 = Sphere::new(5);
        let intersection5 = Intersection::new(1.0, shape5.clone());
        let comps5 = intersection5.prepare_computation(ray5);
        assert_eq!(comps5.point, create_point(0.0, 0.0, 1.0));
        assert_eq!(comps5.eyev, create_vector(0.0, 0.0, -1.0));
        assert!(comps5.inside);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps5.normalv, create_vector(0.0, 0.0, -1.0));
    }
}
