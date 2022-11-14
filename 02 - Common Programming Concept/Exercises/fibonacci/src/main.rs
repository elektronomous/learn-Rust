fn main() {
    generate_fibonacci(11);
}

fn generate_fibonacci(mut n: i32) {
    let mut start_position = 0;
    let mut index = 1;
    let mut temp_result = 0;

    while n != 0 {
        if start_position == 0 && index == 1 {
            print!("{} ", start_position);
        } else {
            print!("{} ", start_position);
        }

        temp_result = start_position + index;
        start_position = index;
        index = temp_result;

        n -= 1;
    }
}
