use std::rc::Rc;

use utils::{hittable::HittableList, sphere::Sphere, vec3::Point3, camera::Camera};


#[warn(non_snake_case)]
fn main() {
        //World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);
}
