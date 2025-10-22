fn main() {
    let x: i8 = 42;
    let y: i16 = 42;
    let z: i32 = 42;
    let a: i64 = 42;
    let b: i128 = 42;
    let c: isize = 42;

    println!("x: {}({})", x, std::any::type_name_of_val(&x));
    println!("y: {}({})", y, std::any::type_name_of_val(&y));
    println!("z: {}({})", z, std::any::type_name_of_val(&z));
    println!("a: {}({})", a, std::any::type_name_of_val(&a));
    println!("b: {}({})", b, std::any::type_name_of_val(&b));
    println!("c: {}({})", c, std::any::type_name_of_val(&c));

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("x: {} y: {} z: {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {} six_point_four: {} one: {}", five_hundred, six_point_four, one);
}
