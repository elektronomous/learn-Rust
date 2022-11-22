fn main() {

    // tuple
    let tup = (100, 20, 30, 40, 50);
    let n = 0;
    println!("{}", tup[0]);
   

    // array
    let arr: [i32; 5] = [10,20,30,40,50];
    let mut index = 0;

    while index < 4 {
        println!("{}", arr[{index += 1; index}]);
    }

    for element in arr.iter() {
        println!("{}", element);
    }

    
    // using loop {} to loop
    let mut counter = 0;
    let result = loop { 
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result: {}", result);

    let mut n_factorial = 5;
    let mut start_counting = 1;

    let factorial_result = loop {
        start_counting *= n_factorial;
        
        n_factorial -= 1;

        if n_factorial == 0 {
            break start_counting;
        }
    };

    println!("The factorial of 5: {}", factorial_result);

    // using while condition to looping
    while n_factorial < 5 {
        n_factorial += 1;
        println!("{}", n_factorial);
    }
    
    // using for loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("each element : {}", element);
    }

    for element in 1..4 {
        println!("each element : {}", element);
    }
}
