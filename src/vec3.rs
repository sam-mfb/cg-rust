use num_traits::{Float, NumCast, ToPrimitive};

pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,

    pub z: T,
}

impl<T: Float + Default> Vec3<T> {
    pub fn new() -> Self {
        Vec3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }

    fn splat<U: ToPrimitive + Copy>(val: U) -> Self {
        Vec3 {
            x: NumCast::from(val).unwrap(),
            y: NumCast::from(val).unwrap(),
            z: NumCast::from(val).unwrap(),
        }
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Vec3<T> {
        let len_sq = self.dot(self);
        if len_sq > T::zero() {
            let inv = T::one() / len_sq.sqrt();
            Vec3 {
                x: self.x * inv,
                y: self.y * inv,
                z: self.z * inv,
            }
        } else {
            Vec3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
}

impl<T: Float, U: ToPrimitive> From<(U, U, U)> for Vec3<T> {
    fn from((x, y, z): (U, U, U)) -> Self {
        Vec3 {
            x: NumCast::from(x).unwrap(),
            y: NumCast::from(y).unwrap(),
            z: NumCast::from(z).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_zero_vec() {
        let vec3: Vec3<f32> = Vec3::new();
        assert_eq!(vec3.x, 0.0);
        assert_eq!(vec3.y, 0.0);
        assert_eq!(vec3.z, 0.0);
    }

    #[test]
    fn create_broadcast_vec() {
        let vec3: Vec3<f64> = Vec3::splat(11);
        assert_eq!(vec3.x, 11.0);
        assert_eq!(vec3.y, 11.0);
        assert_eq!(vec3.z, 11.0);
    }

    #[test]
    fn create_full_vec() {
        let vec3: Vec3<f32> = Vec3::from((1, 2, 3));
        assert_eq!(vec3.x, 1.0);
        assert_eq!(vec3.y, 2.0);
        assert_eq!(vec3.z, 3.0);
    }

    #[test]
    fn calculate_length() {
        let vec3: Vec3<f32> = (3, 4, 5).into();
        assert_eq!(vec3.length(), (3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt())
    }

    #[test]
    fn calculate_dot_prod_perpendicular() {
        let vec3_one: Vec3<f32> = (0, 0, 1).into();
        let vec3_two: Vec3<f32> = (0, 2, 0).into();
        let prod = vec3_one.dot(&vec3_two);
        assert_eq!(prod, 0.0)
    }

    #[test]
    fn length_of_unit_vectors() {
        let unit_x: Vec3<f32> = (1.0, 0.0, 0.0).into();
        let unit_y: Vec3<f32> = (0.0, 1.0, 0.0).into();
        let unit_z: Vec3<f32> = (0.0, 0.0, 1.0).into();

        assert_eq!(unit_x.length(), 1.0);
        assert_eq!(unit_y.length(), 1.0);
        assert_eq!(unit_z.length(), 1.0);
    }

    #[test]
    fn length_345_triangle() {
        let vec: Vec3<f32> = (3.0, 4.0, 0.0).into();
        assert_eq!(vec.length(), 5.0);
    }

    #[test]
    fn length_known_3d() {
        let vec: Vec3<f32> = (2.0, 3.0, 6.0).into();
        assert_eq!(vec.length(), 7.0);
    }

    #[test]
    fn dot_parallel_vectors() {
        let a: Vec3<f32> = (1.0, 0.0, 0.0).into();
        let b: Vec3<f32> = (1.0, 0.0, 0.0).into();
        assert_eq!(a.dot(&b), 1.0);
    }

    #[test]
    fn dot_opposite_vectors() {
        let a: Vec3<f32> = (1.0, 0.0, 0.0).into();
        let b: Vec3<f32> = (-1.0, 0.0, 0.0).into();
        assert_eq!(a.dot(&b), -1.0);
    }

    #[test]
    fn dot_known_values() {
        let a: Vec3<f32> = (1.0, 2.0, 3.0).into();
        let b: Vec3<f32> = (4.0, 5.0, 6.0).into();
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn normalize_creates_unit_vector() {
        let vec: Vec3<f32> = (3.0, 4.0, 0.0).into();
        let normalized = vec.normalize();
        assert_eq!(normalized.length(), 1.0);
    }

    #[test]
    fn normalize_preserves_direction() {
        let vec: Vec3<f32> = (3.0, 4.0, 0.0).into();
        let normalized = vec.normalize();
        // Normalized vector should be (0.6, 0.8, 0.0)
        // since 3/5 = 0.6 and 4/5 = 0.8
        assert_eq!(normalized.x, 0.6);
        assert_eq!(normalized.y, 0.8);
        assert_eq!(normalized.z, 0.0);
    }

    #[test]
    fn normalize_already_normalized() {
        let unit: Vec3<f32> = (1.0, 0.0, 0.0).into();
        let normalized = unit.normalize();
        assert_eq!(normalized.x, 1.0);
        assert_eq!(normalized.y, 0.0);
        assert_eq!(normalized.z, 0.0);
        assert_eq!(normalized.length(), 1.0);
    }

    #[test]
    fn normalize_arbitrary_vector() {
        let vec: Vec3<f32> = (2.0, 3.0, 6.0).into();
        let normalized = vec.normalize();
        // Length is 7, so normalized should be (2/7, 3/7, 6/7)
        let expected_x = 2.0 / 7.0;
        let expected_y = 3.0 / 7.0;
        let expected_z = 6.0 / 7.0;

        assert!((normalized.x - expected_x).abs() < 1e-6);
        assert!((normalized.y - expected_y).abs() < 1e-6);
        assert!((normalized.z - expected_z).abs() < 1e-6);
        assert!((normalized.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cross_product_unit_vectors() {
        // i × j = k
        let i: Vec3<f32> = (1.0, 0.0, 0.0).into();
        let j: Vec3<f32> = (0.0, 1.0, 0.0).into();
        let k = i.cross(&j);
        assert_eq!(k.x, 0.0);
        assert_eq!(k.y, 0.0);
        assert_eq!(k.z, 1.0);

        // j × k = i
        let k_vec: Vec3<f32> = (0.0, 0.0, 1.0).into();
        let result = j.cross(&k_vec);
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);

        // k × i = j
        let result = k_vec.cross(&i);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 1.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn cross_product_anticommutative() {
        // a × b = -(b × a)
        let a: Vec3<f32> = (1.0, 2.0, 3.0).into();
        let b: Vec3<f32> = (4.0, 5.0, 6.0).into();

        let cross_ab = a.cross(&b);
        let cross_ba = b.cross(&a);

        assert_eq!(cross_ab.x, -cross_ba.x);
        assert_eq!(cross_ab.y, -cross_ba.y);
        assert_eq!(cross_ab.z, -cross_ba.z);
    }

    #[test]
    fn cross_product_parallel_vectors() {
        // Cross product of parallel vectors is zero
        let a: Vec3<f32> = (1.0, 2.0, 3.0).into();
        let b: Vec3<f32> = (2.0, 4.0, 6.0).into(); // b = 2*a

        let result = a.cross(&b);
        assert!((result.x).abs() < 1e-6);
        assert!((result.y).abs() < 1e-6);
        assert!((result.z).abs() < 1e-6);
    }

    #[test]
    fn cross_product_known_values() {
        // (1,2,3) × (4,5,6) = (2*6-3*5, 3*4-1*6, 1*5-2*4)
        //                    = (12-15, 12-6, 5-8)
        //                    = (-3, 6, -3)
        let a: Vec3<f32> = (1.0, 2.0, 3.0).into();
        let b: Vec3<f32> = (4.0, 5.0, 6.0).into();

        let result = a.cross(&b);
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn cross_product_perpendicular() {
        // Result should be perpendicular to both input vectors
        let a: Vec3<f32> = (1.0, 2.0, 3.0).into();
        let b: Vec3<f32> = (4.0, 5.0, 6.0).into();

        let result = a.cross(&b);

        // dot product with perpendicular vectors is 0
        assert!((result.dot(&a)).abs() < 1e-5);
        assert!((result.dot(&b)).abs() < 1e-5);
    }
}
