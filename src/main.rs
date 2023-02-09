mod tuple;

use crate::tuple::*;

fn main()
{
    let a = create_point(4.3, -4.2, 3.1);
    println!("a.x {} a.y {} a.z {}", a.x, a.y, a.z);
}
