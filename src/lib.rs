pub mod geometry;
pub mod render;

#[cfg(test)]
mod tests {
    use crate::geometry::vec3::Vec3;

    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(-4.0, 15.0, 7.0), vec1 + vec2);
    }

    #[test]
    fn test_vec3_sub() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::new(-3.0, -3.0, -3.0), vec1 - vec2);
    }

    #[test]
    fn test_vec3_mul() {
        let vec1 = Vec3::new(1.0, 2.0, 4.0);
        let vec2 = Vec3::new(1.0, 0.5, 0.25);
        assert_eq!(Vec3::new(1.0, 1.0, 1.0), vec1 * vec2);
    }

    #[test]
    fn test_vec3_div() {
        let vec1 = Vec3::new(10.0, 20.0, 40.0);
        let vec2 = Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(Vec3::new(10.0, 10.0, 10.0), vec1 / vec2);
    }

    #[test]
    fn test_vec3_scalar() {
        let vec1 = Vec3::new(10.0, 20.0, 40.0);
        let vec2 = Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(10.0 + 2.0 * 20.0 + 4.0 * 40.0, vec1 % vec2);
    }
}
