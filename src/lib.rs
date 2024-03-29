pub mod camera;
pub mod canvas;
pub mod intersections;
pub mod lights;
pub mod materials;
pub mod matrices;
pub mod patterns;
pub mod rays;
pub mod shapes;
pub mod transformations;
pub mod tuples;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
