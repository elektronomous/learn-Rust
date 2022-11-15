fn main() {
    let my_age = 27;

    if my_age < 20 {
        println!("Alhamdulillah, Allah still gives you a life");
    } else if my_age < 25 {
        println!("Allah give you a young look");
    } else if my_age > 26 {
        println!("Be wise in deciding something");
    }

    let am_i_handsome = true;
    let rate_myself = if am_i_handsome {
        "100%"
    } else {
        "50%"
    };

    println!("rate myself: {}", rate_myself);
}
