use lib::camera::Camera;
use lib::lights::PointLight;
use lib::materials::Material;
use lib::shapes::Shape;
use lib::transformations::Transformation;
use lib::tuples::Tuple;
use lib::world::World;
use std::f64::consts::*;
use std::path::Path;

fn main() {
    let mut floor = Shape::sphere();
    floor.set_transform(Transformation::scaling(10., 0.01, 10.));
    floor.material = Material::new();
    floor.material.color = Tuple::color(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = Shape::sphere();
    left_wall.set_transform(
        Transformation::translation(0., 0., 5.)
            * Transformation::rotation_y(-FRAC_PI_4)
            * Transformation::rotation_x(FRAC_PI_2)
            * Transformation::scaling(10., 0.01, 10.),
    );
    left_wall.material = floor.material;

    let mut right_wall = Shape::sphere();
    right_wall.set_transform(
        Transformation::translation(0., 0., 5.)
            * Transformation::rotation_y(FRAC_PI_4)
            * Transformation::rotation_x(FRAC_PI_2)
            * Transformation::scaling(10., 0.01, 10.),
    );
    right_wall.material = floor.material;

    let mut middle = Shape::sphere();
    middle.set_transform(Transformation::translation(-0.5, 1., 0.5));
    middle.material = Material::new();
    middle.material.color = Tuple::color(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::sphere();
    right.set_transform(
        Transformation::translation(1.5, 0.5, -0.5) * Transformation::scaling(0.5, 0.5, 0.5),
    );
    right.material = Material::new();
    right.material.color = Tuple::color(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::sphere();
    left.set_transform(
        Transformation::translation(-1.5, 0.33, -0.75) * Transformation::scaling(0.33, 0.33, 0.33),
    );
    left.material = Material::new();
    left.material.color = Tuple::color(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light_source = PointLight::new(Tuple::point(-10., 10., -10.), Tuple::color(1., 1., 1.));

    let world = World::with_objects_and_light(
        vec![floor, left_wall, right_wall, middle, right, left],
        light_source,
    );

    let mut camera = Camera::new(100, 50, FRAC_PI_3);
    camera.set_transform(Transformation::view(
        &Tuple::point(0., 1.5, -5.),
        &Tuple::point(0., 1., 0.),
        &Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(&world);
    canvas.save_to_file(Path::new("scene.ppm")).unwrap();
}
