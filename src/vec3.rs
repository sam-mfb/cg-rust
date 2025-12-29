pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T:Default> Vec3<T> {
    pub fn new() -> Self {
        Vec3 {
            x : T::default(),
            y: T::default(),
            z : T::default()
        }
    }
}

impl<T:Copy> From<T> for Vec3<T> {
    fn from(val:T) -> Self {
        Vec3 {
            x: val,
            y: val,
            z: val
        }
    }
}

impl<T> From<(T,T,T,)> for Vec3<T> {
    fn from((x,y,z):(T,T,T)) -> Self {
        Vec3 {x,y,z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_zero_vec() {
        let vec3:Vec3<f32> = Vec3::new();
        assert_eq!(vec3.x,0.0);
        assert_eq!(vec3.y,0.0);
        assert_eq!(vec3.z,0.0);
    }

    #[test]
    fn create_broadcast_vec() {
        let vec3:Vec3<u16> = Vec3::from(11);
        assert_eq!(vec3.x,11);
        assert_eq!(vec3.y,11);
        assert_eq!(vec3.z,11);
    }
    
    #[test]
    fn create_full_vec() {
        let vec3:Vec3<u16> = Vec3::from((1,2,3));
        assert_eq!(vec3.x,1);
        assert_eq!(vec3.y,2);
        assert_eq!(vec3.z,3);
    }
}
