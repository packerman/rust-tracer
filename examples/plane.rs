use lib::{
    camera::Camera, lights::PointLight, materials::Material, shapes::Shape,
    transformations::Transformation, tuples::Tuple, world::World,
};
use std::{f64::consts::*, path::Path};

fn main() {
    let mut floor = Shape::plane();
    *floor.material_mut() = Material::default();
    floor.material_mut().set_color(Tuple::color(1., 0.9, 0.9));
    floor.material_mut().specular = 0.;

    let mut middle = Shape::sphere();
    middle.set_transform(Transformation::translation(-0.5, 1., 0.5));
    *middle.material_mut() = Material::default();
    middle.material_mut().set_color(Tuple::color(0.1, 1., 0.5));
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Shape::sphere();
    right.set_transform(
        Transformation::translation(1.5, 0.5, -0.5) * Transformation::scaling(0.5, 0.5, 0.5),
    );
    *right.material_mut() = Material::default();
    right.material_mut().set_color(Tuple::color(0.5, 1., 0.1));
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Shape::sphere();
    left.set_transform(
        Transformation::translation(-1.5, 0.33, -0.75) * Transformation::scaling(0.33, 0.33, 0.33),
    );
    *left.material_mut() = Material::default();
    left.material_mut().set_color(Tuple::color(1., 0.8, 0.1));
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;

    let light_source = PointLight::new(Tuple::point(-10., 10., -10.), Tuple::color(1., 1., 1.));

    let world = World::with_objects_and_light(vec![floor, middle, right, left], light_source);

    let mut camera = Camera::new(800, 600, FRAC_PI_3);
    camera.set_transform(Transformation::view(
        &Tuple::point(0., 1.5, -5.),
        &Tuple::point(0., 1., 0.),
        &Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(&world);
    canvas.save_to_file(Path::new("plane.ppm")).unwrap();
}
