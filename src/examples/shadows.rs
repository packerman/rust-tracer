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
    let mut camera = Camera::new(400, 200, 0.524);
    camera.set_transform(Transformation::view(
        &Tuple::point(40., 0., -70.),
        &Tuple::point(0., 0., -5.),
        &Tuple::vector(0., 1., 0.),
    ));

    let light = PointLight::new(Tuple::point(0., 0., -100.), Tuple::color(1., 1., 1.));

    let mut sphere_material = Material::new();
    sphere_material.ambient = 0.2;
    sphere_material.diffuse = 0.8;
    sphere_material.specular = 0.3;
    sphere_material.shininess = 200.;

    let mut wrist_material = sphere_material.clone();
    wrist_material.color = Tuple::color(0.1, 1., 1.);

    let mut palm_material = sphere_material.clone();
    palm_material.color = Tuple::color(0.1, 0.1, 1.);

    let mut thumb_material = sphere_material.clone();
    thumb_material.color = Tuple::color(0.1, 0.1, 1.);

    let mut index_material = sphere_material.clone();
    index_material.color = Tuple::color(1., 1., 0.1);

    let mut middle_material = sphere_material.clone();
    middle_material.color = Tuple::color(0.1, 1., 0.5);

    let mut ring_material = sphere_material.clone();
    ring_material.color = Tuple::color(0.1, 1., 0.1);

    let mut pinky_material = sphere_material.clone();
    pinky_material.color = Tuple::color(0.1, 0.5, 1.);

    let mut backdrop = Shape::sphere();
    backdrop.material.color = Tuple::color(1., 1., 1.);
    backdrop.material.ambient = 0.;
    backdrop.material.diffuse = 0.5;
    backdrop.material.specular = 0.;
    backdrop.set_transform(
        Transformation::translation(0., 0., 20.) * Transformation::scaling(200., 200., 0.01),
    );

    let mut wrist = Shape::sphere();
    wrist.material = wrist_material;
    wrist.set_transform(
        Transformation::rotation_z(FRAC_PI_4)
            * Transformation::translation(-4., 0., -21.)
            * Transformation::scaling(3., 3., 3.),
    );

    let mut palm = Shape::sphere();
    palm.material = palm_material;
    palm.set_transform(
        Transformation::translation(0., 0., -15.) * Transformation::scaling(4., 3., 3.),
    );

    let mut thumb = Shape::sphere();
    thumb.material = thumb_material;
    thumb.set_transform(
        Transformation::translation(-2., 2., -16.) * Transformation::scaling(1., 3., 1.),
    );

    let mut index = Shape::sphere();
    index.material = index_material;
    index.set_transform(
        Transformation::translation(3., 2., -22.) * Transformation::scaling(3., 0.75, 0.75),
    );

    let mut middle = Shape::sphere();
    middle.material = middle_material;
    middle.set_transform(
        Transformation::translation(4., 1., -19.) * Transformation::scaling(3., 0.75, 0.75),
    );

    let mut ring = Shape::sphere();
    ring.material = ring_material;
    ring.set_transform(
        Transformation::translation(4., 0., -18.) * Transformation::scaling(3., 0.75, 0.75),
    );

    let mut pinky = Shape::sphere();
    pinky.material = pinky_material;
    pinky.set_transform(
        Transformation::translation(3., -1.5, -20.)
            * Transformation::rotation_z(-PI / 10.)
            * Transformation::translation(1., 0., 0.)
            * Transformation::scaling(2.5, 0.6, 0.6),
    );

    let world = World::with_objects_and_light(
        vec![backdrop, wrist, palm, thumb, index, middle, ring, pinky],
        light,
    );

    let canvas = camera.render(&world);
    canvas.save_to_file(Path::new("shadows.ppm")).unwrap();
}
