use std::fmt::{self, Formatter, Display};

pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,


}


impl Display for Vec3{
    fn fmt (&self, f: &mut Formatter) -> fmt::Result{


        write!(f, "{}, {}, {}", self.x, self.y, self.z)    

    }
    
}

pub fn new_vec(x: i32, y:i32, z:i32) -> Vec3{
    Vec3{
        x: x,
        y: y,
        z: z,
    }
} 

pub fn cross_prod(vector1: Vec3, vector2: Vec3) -> Vec3{

    new_vec((vector1.y * vector2.z - vector1.z * vector2.y),
    (vector1.z * vector2.x - vector1.x * vector2.z),
    (vector1.x * vector2.y - vector1.y * vector2.x))


}