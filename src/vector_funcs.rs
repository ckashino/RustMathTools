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