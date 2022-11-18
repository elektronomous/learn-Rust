fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1;                        // after this you can't longer access the s1
                                            // because s1 is moved to s2

    s2.push_str(", world!");
    println!("{}", s2);

    let _s = String::from("Hay");           // _s's comes into scope

    takes_ownership(_s);                    // _s's value moves into the function
                                            // and so is no longer valid here

    let _my_number = 5;                     // _my_number comes into scope
    makes_copy(_my_number);                 // _my_number would move into the function,
                                            // but i32 is `Copy`, so it's okay to
                                            // still use x afterward.
}

fn takes_ownership (some_string: String) {  // some_string comes into scope
    println!("{}", some_string);

} // Here, some_string goes out of scope and `Drop` is called. The backing 
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);

} // Here, some_integer goes out of scope and Nothing special happens.