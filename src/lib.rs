pub mod tuples;
pub mod canvas;
pub mod matrices;
pub mod transformations;
pub mod rays;
pub mod spheres;
pub mod intersections;
pub mod lights;
pub mod materials;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
