fn main() {
    let (x, y, z) = (1, 2, 3);
    
    // using {} as an expression
    let total_of_x_y_z = {
        x + y + z
    };

    println!("{} {} {}", x, y, z);
    println!("total: {}", total_of_x_y_z);
    println!("Hello, world!");
    another_function();
    accept_arguments(15);

    let total = sum(10, 20);
    println!("the result: {}", total);
    println!("five(): {}", five());
}

fn another_function() {
    println!("another function");
}

// function that accept arguments
fn accept_arguments(x: i32) {
    println!("the argument of x: {}", x);
}

// function with return values
fn five() -> i32 {
    5
}

fn sum(x: i32, y: i32) -> i32 {
    // we dont give this expression a semicolon(;)
    // to make them return implicitly
    x + y
}
