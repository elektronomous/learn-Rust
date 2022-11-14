fn main() {
    let (x, y, z) = (1, 2, 3);
    println!("{} {} {}", x, y, z);
    println!("Hello, world!");
    another_function();
    accept_arguments(15);

    let result = sum(10, 20);
    println!("the result: {}", result);
}

fn another_function() {
    println!("another function");
}

fn accept_arguments(x: i32) {
    println!("the argument of x: {}", x);
}

fn sum(x: i32, y: i32) {
    x + y;
}
