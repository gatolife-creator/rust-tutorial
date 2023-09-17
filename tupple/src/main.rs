fn main() {
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup1;
    println!("The value of y is: {}", y);

    let tup2 = (500, 6.4, 1);

    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;

    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);
}
