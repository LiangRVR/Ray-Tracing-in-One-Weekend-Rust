use crate::{ vec3::Point3, hittable::{ Hittable, HitRecord }, ray::Ray };

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().lengt_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.lengt_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
