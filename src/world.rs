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
    }
}
