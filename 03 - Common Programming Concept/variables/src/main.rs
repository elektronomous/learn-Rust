fn main() {
    let _s = "Hello";

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x now: {}", x);

    // shadowing
    let my_number = 10;
    println!("The value of my number: {}", my_number);

    let my_number = 20;
    println!("The value of my number: {}", my_number);

    let my_number = my_number + 10;
    println!("the value of my number: {}", my_number);

    let my_number = "Hello World";
    println!("the value of my number: {}", my_number);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("the spaces: {}", spaces);

    /* make the compiler panic: uncomment this out
    let overflow:i8 = 256;
    println!("the overflow: {}", overflow);
    */

    // specify an integer
    let my_integer: u32 = 10;

    // specify a float number
    let phi: f64 = 3.14;

    println!("my_number and phi: {} {}", my_integer, phi);

    // specify the boolean type
    let am_i_handsome: bool = true;

    // specified a tuple
    let tup: (u32, f64, bool) = (500, 22.5, true);
    // you can destructuring it
    let (_x, _y, _z) = tup;
    // also like this
    let five_hundred = tup.0;
    let _phi = tup.1;

    // specified an array
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr = [3; 5];
    let _days = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];

    let [_one, _two, _three, _four, _five] = _arr;
    println!("{}{}{}", _one, _two, _three);
}
