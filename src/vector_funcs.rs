use std::fmt::{self, Formatter, Display};

#[derive(Copy, Clone)]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,


}


impl Display for Vec3{
    fn fmt (&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "({}, {}, {})", self.x, self.y, self.z)    
    }    
}
impl std::ops::Add for Vec3 {
    type  Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {x: self. x + other.x, y: self.y + other.y, z: self.z + other.z}
    }


}
impl std::ops::Sub for Vec3 {
    type  Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {x: (self.x) - (other.x), y: (self.y) - (other.y), z: (self.z) - (other.z)}
    }


}
pub fn new_vec(x: i32, y:i32, z:i32) -> Vec3{
    Vec3{
        x: x,
        y: y,
        z: z,
    }
} 

pub fn cross_prod(v1: Vec3, v2: Vec3) -> Vec3{
    new_vec((v1.y * v2.z - v1.z * v2.y),
    (v1.z * v2.x - v1.x * v2.z),
    (v1.x * v2.y - v1.y * v2.x))
}

pub fn dot_prod(v1: Vec3, v2: Vec3) -> i32{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

