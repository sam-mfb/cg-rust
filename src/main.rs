mod vec3;
use vec3::Vec3;

fn main() {
    let vec3: Vec3<f32> = Vec3::new();
    println!("{} {} {}", vec3.x, vec3.y, vec3.z);
}
