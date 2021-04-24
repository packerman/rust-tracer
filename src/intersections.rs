use crate::spheres::Sphere;

pub struct Intersection<'a> {
    t: f32,
    object: &'a Sphere,
}

impl Intersection<'_> {

    pub fn new(t: f32, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        return self.object
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::ptr;

    #[test]
    fn creating_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t(), 3.5);
        assert!(ptr::eq(i.object(), &s));
    }
}
