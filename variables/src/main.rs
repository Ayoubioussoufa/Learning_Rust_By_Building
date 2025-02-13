// fn main() {
//     let x = 5;
//     println!("x : {x}");
//     let x = x + 1; // shadowing even if its immutables
//     println!("x : {x}");
//     {
//         let x = x * 2;
//         println!("x : {x}");
//     }
//     println!("x : {x}");
//     println!("x : {x}");
//     let spaces = "   ";
//     println!("spaces : {spaces}|");

//     let spaces = spaces.len();
//     println!("spaces : {spaces}");
//     // we can even change the type of the variable by shadowing
//     let a = [0; 5];
//     println!("{:?}", a);
// }

// const are always immutable
// const can be declared in any score, even global one not let
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// Rust’s naming convention for constants is to use all uppercase with underscores between words

// 2 data types in Rust: scalar and compound

/* Scalar : represents a single value, int, float, bool, characters
    i8 -> i128
    u8 -> u128
    arch : isize -> usize types depend on the architecture of the computer
    f32 f64 float point numbers

*/

/* Compound types:  can group multiple values into one type = tuples and arrays.
   -----TUPLE-----
   let tup: (i32, f64, u8) = (500, 6.4, 1);
   let (x, y, z) = tup;
   let five_hundred = tup.0;

   let six_point_four = tup.1;

   let one = tup.2;
   -----ARRAY-----
   data allocated on the stack
   An array isn’t as flexible as the vector type, though.
   A vector is a similar collection type provided by the standard library
   that is allowed to grow or shrink in size.
   let a: [i32; 5] = [1, 2, 3, 4, 5];
   let a = [0; 5] means array of length 5 and all elements has value 0
*/

//  use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");

//     let mut x = five();

//     println!("{}", x);

//     println!("{}", plus_one(&mut x));
//     println!("{}", plus_one(&mut x));
//     println!("{}", plus_one(&mut x));
// }

// fn five() -> i32 {
//     5
// }

// fn plus_one(x: &mut i32) -> i32 {
//     *x += 1;
//     *x
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         'tz: loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break 'tz;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut a = [0; 10];

//     // First, modify the array with a mutable slice
//     let complete = &mut a[..]; // Mutable slice for complete
//     complete[1] = 35;

//     // Now, create an immutable slice for 'middle' after 'complete' is no longer needed
//     let middle = &a[1..4]; // Immutable slice for middle

//     // Print the results
//     println!("{a:?} {complete:?}");
// }

fn main() {
    // Uncomment this block if you want to use the loop.
    // for number in (0..4) {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");

    // let mut a = [0; 10]; // Create a mutable array of 10 zeros.
    // let complete = &mut a[..]; // Create a mutable slice referencing the entire array.
    // complete[1] = 35; // Modify the second element of the array through the mutable slice.

    // // At this point, `complete` goes out of scope, so we can create an immutable slice.
    // let middle = &a[1..4]; // Create an immutable slice referencing elements from index 1 to 3.

    // println!("Array: {:?}", a); // Print the entire array.
    // println!("Middle slice: {:?}", middle); // Print the middle slice.
    let mut x = 10;
    {
        let r1 = &mut x; // Mutable reference
        *r1 += 1;
    } // `r1` goes out of scope here
    let r2 = &x; // Now, an immutable reference is allowed
    println!("{}", r2); // Works fine
}


// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for (index, element) in a.iter().enumerate() {
//         println!("the value is: {index}, {element}");
//     }
// }
