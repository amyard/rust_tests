
use std::io;

fn main() {
    
    // VARIABLES
    let mut x = 5;
    println!("The value of x is {x}.");
    x = 6;
    println!("The value of x is {x}.");
    
    // CONSTANTS 
    //  are always immutable. We aren't allowed to use -mut-.
    // type must be annotated. can be declared in any scope even in global.
    // may be set only for constant expression, not the result of value tant could be computed at runtime.
    const THEE_SOMETHING: u32 = 60 * 60 * 3;
    println!("The value of constant is {THEE_SOMETHING}.");

    // SHADOWING
    // Declare a new variable with the same name as a previous variable.
    // first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used.
    let a1 = 5;
    let a1 = a1 + 1;
    {
        let a1 = a1 * 2;
        println!("the value of -a1- in inner scope is {}.", a1);  // result = 12
    }
    println!("the value of -a1- is {}.", a1);   // result = 6
    
    // shadowing and mut ---> difference
    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // Shadowing ---> we can change the type of the value but reuse the same name. 
    
    // Correct
    // let a2 = "  ";
    // let a2 = s2.len();
    
    // WRONG ---> will get an error
    // let mut a3 = "  ";
    // a3 = a3.len();
    
    
    // DATA TYPES - scalar and compound
    // SCALAR - integers, floating-point numbers, Booleans, and characters
    // COMPOUND types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // 1
    let tup: (i32, f64, u8) = (500, 60.1, 8);
    
    // 2
    let tup1 = (500,10,'a', "asd");
    let (x,y,z,z1) = tup1; //This is called destructuring, because it breaks the single tuple into three parts. 
    
    // 3
    let a: (i32, f64, u8) = (500, 60.1, 8);
    let a1 = a.0;
    let a2 = a.1;
    let a3 = a.2;
    
    // ARRAY - Unlike a tuple, every element of an array must have the same type. Arrays in Rust have a fixed length.
    let col = [1,2,3,4];
    // An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    
    // 1
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 2 
    let a = [3; 5];   // the same as    let a = [3, 3, 3, 3, 3];

    //test_array();

    clean_lines();
    
    test_func(5);
    print_labeled_measurement(15, 's');
    
    let r0 = {
        let r1 = 3;
        r1 + 1   // IMPORTANT ---> to return the result DO NOT USE semicolon in the end of expression.
    };
    
    println!("the r value is {r0}.");
    
    // FUNC  with return value
    let n1 = return_value_func();
    println!("the return_value_func is {}.", n1);
    
    // IF Condition
    let condition = true;
    let nmb = if condition { 5 } else { 6 };
    
    println!("The value 'nmb' is {}.", nmb);
    
    // LOOP
    clean_lines();
    
    let mut count = 0;
    'counting_up: loop {    // LOOP LABEL
        println!("count = {}.", count);
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}.", remaining);
            if remaining == 9 {
                break;
            }
            if count == 4 {
                break 'counting_up;   // To finish the general looping.
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}.", count);
    
    
    clean_lines();
    
    let mut counter1 = 0;
    let result = loop {
        counter1 += 1;
        
        if counter1 == 10 {
            break counter1 * 2;
        }
    };
    
    println!("The result of loop is {}.", result);
    
    
    // WHILE 
    clean_lines();
    
    let mut nmb1 = 3;
    
    while nmb1 != 0 {
        println!("While looping - {}!", nmb1);
        nmb1 -= 1;
    } 
    
    println!("LIFTOFF!!!");
    
    
    // Looping through the collection
    clean_lines();
    
    let arr1 = [10,20,30,40,50];
    let mut index = 0;
    
    while index < 5 {
        println!("The value is: {}.", arr1[index]);
        index += 1;
    }
    
    for element in arr1 {
        println!("The value is: {}.", element);
    }
    
    clean_lines();
    
    for numb in (1..4).rev() {
        println!("{}!", numb);
    }
    
    println!("LIFTOFF!!!");
    
    clean_lines();
}

fn clean_lines() {
    println!("-------------------------------------------------------");
    println!("-------------------------------------------------------");
    println!("-------------------------------------------------------");
}

fn return_value_func() -> i32{
    5
}

fn test_func(some_value: i32) {
    println!("test_func value is {}.", some_value);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}


/*
fn test_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
*/
