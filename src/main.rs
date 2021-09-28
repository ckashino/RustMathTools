mod vector_funcs;

use vector_funcs::*;

fn main() {


    let mut p = new_vec(1, 1, 1);
    let mut q = new_vec(2, -3, 6);

    let cp = cross_prod(p, q);
    let dp = dot_prod(p,q);

    println!("{}", p.magnitude());

}
