mod render;
pub(crate) mod geometry;

#[cfg(test)]
mod tests {
    use crate::geometry::vec3::Vec3;

    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(-4.0, 15.0, 7.0), vec1 + vec2);
    }
}
