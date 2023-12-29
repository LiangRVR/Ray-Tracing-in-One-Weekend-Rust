use crate::{
    hittable::{ Hittable, HitRecord },
    ray::Ray,
    interval::Interval,
    utils::INFINITY,
    color::Color,
    vec3::{ Point3, Vec3 },
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    camera_center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            camera_center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {

        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixer_center =
                    self.pixel00_loc +
                    (i as f64) * self.pixel_delta_u +
                    (j as f64) * self.pixel_delta_v;
                let ray_direction = pixer_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color = Camera::ray_color(&r, world);
                pixel_color.write_color();
            }
        }
        eprintln!("\rDone.                               \n");
    }

    fn initialize(&mut self,){
        //Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);

        // Camera
        self.camera_center = Point3::new(0.0, 0.0, 0.0);

        //Determine Viewport Dimensions
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        //Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);


    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        let world_interval = Interval { min: 0.0, max: INFINITY };
        if world.hit(r, &world_interval, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
