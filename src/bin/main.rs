use std::rc::Rc;
use utils::{
    color::Color,
    vec3::Point3,
    vec3::Vec3,
    ray::Ray,
    hittable::{ HitRecord, Hittable, HittableList },
    constants::INFINITY,
    sphere::Sphere,
};

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
#[warn(non_snake_case)]
fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    //Calculate the image height, and ensure that it's at least 1.
    let mut image_height = ((image_width as f64) / aspect_ratio) as i32;
    image_height = image_height.max(1);

    //World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    //Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("Scanlines remaining {}", image_height - j);
        for i in 0..image_width {
            let pixer_center =
                pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray_direction = pixer_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color();
        }
    }
    eprintln!("\rDone.                               \n");
}
