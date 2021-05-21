use lib::planes::Plane;
use lib::shapes::Shape;
use std::path::Path;
use lib::camera::Camera;
use std::f64::consts::*;
use lib::world::World;
use lib::lights::PointLight;
use lib::tuples::Tuple;
use lib::materials::Material;
use lib::transformations::Transformation;
use lib::spheres::Sphere;

fn main() {
    let mut floor: Box<dyn Shape> = Box::new(Plane::new());
    floor.set_material(Material::new());
    floor.properties_mut().material.color = Tuple::color(1., 0.9, 0.9);
    floor.properties_mut().material.specular = 0.;

    let mut middle: Box<dyn Shape> = Box::new(Sphere::new());
    middle.set_transform(Transformation::translation(-0.5, 1., 0.5));
    middle.set_material(Material::new());
    middle.properties_mut().material.color = Tuple::color(0.1, 1., 0.5);
    middle.properties_mut().material.diffuse = 0.7;
    middle.properties_mut().material.specular = 0.3;

    let mut right: Box<dyn Shape> = Box::new(Sphere::new());
    right.set_transform(Transformation::translation(1.5, 0.5, - 0.5) * Transformation::scaling(0.5, 0.5, 0.5));
    right.set_material(Material::new());
    right.properties_mut().material.color = Tuple::color(0.5, 1., 0.1);
    right.properties_mut().material.diffuse = 0.7;
    right.properties_mut().material.specular = 0.3;

    let mut left: Box<dyn Shape> = Box::new(Sphere::new());
    left.set_transform(Transformation::translation(- 1.5, 0.33, - 0.75) * Transformation::scaling(0.33, 0.33, 0.33));
    left.set_material(Material::new());
    left.properties_mut().material.color = Tuple::color(1., 0.8, 0.1);
    left.properties_mut().material.diffuse = 0.7;
    left.properties_mut().material.specular = 0.3;

    let light_source = PointLight::new(Tuple::point(-10., 10., -10.), Tuple::color(1., 1., 1.));

    let world = World::with_objects_and_light(vec![floor, middle, right, left], light_source);

    let mut camera = Camera::new(1280, 800, FRAC_PI_3);
    camera.set_transform(Transformation::view(&Tuple::point(0., 1.5, -5.),
                                                &Tuple::point(0., 1., 0.),
                                                &Tuple::vector(0., 1., 0.)));

    let canvas = camera.render(&world);
    canvas.save_to_file(Path::new("plane.ppm")).unwrap();
    canvas.save_to_file(Path::new("plane.png")).unwrap();
}
