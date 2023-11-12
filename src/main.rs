// // // // // // //
// excercise: B //
// // // // // //
// use rust_testing::area_of;
// use rust_testing::volume;
// fn main() {
// let width = 4;
//     let height = 7;
//     let depth = 10;
//     let area = area_of(width, height);
//     println!("Area is {}", area);

//     println!("Volume is {}", volume(width, height, depth));
// }

// // // // // // //
// excercise: C //
// // // // // //
// use rust_testing::{ding, on_off, print_array, print_difference, print_distance};

// fn main() {
//     let coords: (f32, f32) = (6.3, 15.0);

//     print_difference(coords.0, coords.1);

//     let coords_arr: [f32; 2] = [coords.0, coords.1];
//     print_array(coords_arr);

//     let series = [1, 1, 2, 3, 5, 8, 13];

//     ding(series[6]);

//     let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
//     on_off(mess.2[1].0);

//     print_distance(coords);
// }

// // // // // // //
// excercise: D //
// // // // // //
// fn main() {
//     let args: Vec<String> = std::env::args().skip(1).collect();

//     for arg in args {
//         if arg == "sum" {
//             sum();
//         } else if arg == "double" {
//             double();
//         } else {
//             count(arg);
//         }
//     }
// }

// fn sum() {
//     let mut sum = 0;
//     for i in 7..=23 {
//         sum += i;
//     }
//     println!("The sum is {}", sum);
// }

// fn double() {
//     let mut count = 0;
//     let mut x = 1;
//     // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
//     // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
//     // with `cargo run double`  Hint: The answer is 9 times.
//     while x <= 500 {
//         x *= 2;
//         count += 1;
//     }

//     println!("You can double x {} times until x is larger than 500", count);
// }

// fn count(arg: String) {
//     // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
//     // You will need to count your loops, somehow.  Run it with `cargo run bananas`
//     //
//     // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
//     let mut count = 0;
//     'printarg: loop {
//         print!("{} ", arg);
//         count += 1;
//         if count == 8 {
//             break 'printarg;
//         }
//     }

//     println!(); // This will output just a newline at the end for cleanliness.
// }

// // // // // // //
// excercise: E //
// // // // // //
// fn main() {
//     // This fancy stuff either gets the first argument as a String, or prints
//     // usage and exits if an argument was not supplied to the program.
//     let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
//         println!("Please supply an argument to this program.");
//         std::process::exit(-1);
//     });

//     let arg2: String = std::env::args().nth(2).unwrap_or_else(|| {
//         println!("Please supply an argument to this program.");
//         return "".to_string();
//     });

//     if arg2 != "" {
//         let (x, y): (i32, i32) = (arg.parse::<i32>().unwrap_or_else(|err| {
//             println!("Argument 1 is not an integer. {}", err);
//             std::process::exit(-1);
//         }), arg2.parse::<i32>().unwrap_or_else(|err| {
//             println!("Argument 2 is not an integer. {}", err);
//             std::process::exit(-1);
//         }));
        
//         println!("{} + {} = {}, even via references", x, y, add(&x, &y));
//         return;
//     }

    

//     inspect(&arg);

//     // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
//     // the String if it doesn't already end with "s". Then uncomment and run the code below with
//     // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
//     //
//     change(&mut arg);
//     println!("I have many {}", arg);

//     // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
//     // indicating whether or not the String both starts with a "b" AND contains an "a".
//     // Hint 1: use `.starts_with("b")` and `.contains("a")`
//     // Hint 2: `&&` is the boolean "AND" operator
//     //
//     if eat(arg) {
//        println!("Might be bananas");
//     } else {
//        println!("Not bananas");
//     }

//     // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

//     // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
//     // ignores what is in the string and replaces the contents of the string with the String
//     // "sparkly". Then uncomment the code below.
//     //
//     // Hint: You will need to dereference the mutable reference in order to assign it a
//     // new value.
//     //
//     let mut material = "mud".to_string();
//     println!("This material is just `{}`.", material);
//     bedazzle(&mut material);
//     println!("Wow! Now the material is `{}`!", material);
// }

// fn inspect(arg : &String) {
//     if arg.ends_with("s") {
//         println!("{} is plural", arg);
//     } else {
//         println!("{} is singular", arg);
//     }
// }

// fn change(arg: &mut String) {
//     if !arg.ends_with("s") {
//         arg.push_str("s");
//     }
// }

// fn eat(arg: String) -> bool {
//     arg.starts_with("b") && arg.contains("a")
// }

// fn bedazzle(arg: &mut String) {
//     *arg = "sparkly".to_string();
// }

// fn add(x: &i32, y: &i32) -> i32{
//     *x + *y
// }

// // // // // // //
// excercise: F //
// // // // // //
// 1. Define a trait named `Bite`
// trait Bite {
//     fn bite(self: &mut Self) {
//         println!("I bite.");
//     }
// }

// fn main() {
//     // Once you finish #1 above, this part should work.
//     let mut carrot = Carrot { percent_left: 100.0 };
//     carrot.bite();
//     println!("I take a bite: {:?}", carrot);

//     let mut grapes: Grapes = Grapes { p_left: 100.0 };
//     grapes.bite();
//     println!("I take a bite: {:?}", grapes);

    
//     // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
//     // function that:
//     // - takes a mutable reference to any type that implements Bite
//     // - calls `.bite()` several times
//     // Hint: Define the generic type between the function name and open paren:
//     //       fn function_name<T: Bite>(...)
//     //
//     bunny_nibbles(&mut carrot);
//     println!("Bunny nibbles for awhile: {:?}", carrot);

//     bunny_nibbles(&mut grapes);
//     println!("Bunny nibbles for awhile: {:?}", grapes);
// }

// fn bunny_nibbles<T: Bite>(veg : &mut T){
//     veg.bite();
// }

// #[derive(Debug)] // This enables using the debugging format string "{:?}"
// struct Carrot {
//     percent_left: f32,
// }

// impl Bite for Carrot {
//     fn bite(self: &mut Self) {
//         // Eat 20% of the remaining carrot. It may take awhile to eat it all...
//         self.percent_left *= 0.8;
//     }
// }
// #[derive(Debug)] 
// struct Grapes {
//     p_left: f32,
// }

// impl Bite for Grapes {
//     fn bite(self: &mut Self){
//         self.p_left -= 1.0;
//     }
// }

// use std::fs::File;


// fn main(){
//     let res = File::open("../../../Cargo.toml");
//     let mut x: Option<i32> = None;
//     x = Some(5);
//     let y = x.unwrap();
//     println!("{}", y);
//     if res.is_ok() {
//         println!("{:?}", res.ok())
//     }
//     else {
//         let z = res.err().unwrap();
        
//         println!("{}", z)
//     }
    

// }

// // // // // // //
// excercise: G //
// // // // // //
// #![allow(unused_variables, unused_mut, dead_code)]

// // Someone is shooting arrows at a target.  We need to classify the shots.
// //
// // 1a. Create an enum called `Shot` with variants:
// // - `Bullseye`
// // - `Hit`, containing the distance from the center (an f64)
// // - `Miss`
// //
// // You will need to complete 1b as well before you will be able to run this program successfully.

// enum Shot {
//     Bullseye,
//     Hit(f64),
//     Miss,
// }


// impl Shot {
//     fn points(self) -> i32 {
//         match self {
//             Shot::Bullseye => {
//                 5
//             }
//             Shot::Hit(x) => {
//                 if x < 3.0 {
//                     2
//                 } else {
//                     1
//                 }
//             }
//             Shot::Miss => {
//                 0
//             }
//         }
//     }
// }

// fn main() {
//     // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
//     let arrow_coords: Vec<Coord> = get_arrow_coords(5);
//     let mut shots: Vec<Shot> = Vec::new();
//     for arrow_coord in arrow_coords {
//         arrow_coord.print_description();
//         let distance = arrow_coord.distance_from_center();
//         if distance < 1.0 {
//             shots.push(Shot::Bullseye)
//         }
//         else if distance < 5.0 {
//             shots.push(Shot::Hit(distance))
//         } else {
//             shots.push(Shot::Miss)
//         }
//     }
//     // 2. For each coord in arrow_coords:
//     //
//     //   A. Call `coord.print_description()`
//     //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
//     //   `coord.distance_from_center()`
//     //      - Less than 1.0 -- `Shot::Bullseye`
//     //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
//     //      - Greater than 5.0 -- `Shot::Miss`


//     let mut total = 0;
//     // 3. Finally, loop through each shot in shots and add its points to total
//     for shot in shots {
//         total += shot.points();
//     }

//     println!("Final point total is: {}", total);
// }

// // A coordinate of where an Arrow hit
// #[derive(Debug)]
// struct Coord {
//     x: f64,
//     y: f64,
// }

// impl Coord {
//     fn distance_from_center(&self) -> f64 {
//         (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
//     }
//     fn print_description(&self) {
//         println!(
//             "coord is {:.1} away, at ({:.1}, {:.1})",
//             self.distance_from_center(),
//             self.x,
//             self.y);
//     }

// }

// // Generate some random coordinates
// fn get_arrow_coords(num: u32) -> Vec<Coord> {
//     let mut coords: Vec<Coord> = Vec::new();
//     for _ in 0..num {
//         let coord = Coord {
//             x: (rand::random::<f64>() - 0.5) * 12.0,
//             y: (rand::random::<f64>() - 0.5) * 12.0,
//         };
//         coords.push(coord);
//     }
//     coords
// }

// // // // // // //
// excercise: H //
// // // // // //
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel::{self, Receiver};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    // 1a. Between the .iter() and the .sum() add a .filter() with a closure to keep any even
    // number (`x % 2` will be 0 for even numbers).
    // 1b. Between the .filter() and the .sum() add a .map() with a closure to square the values
    // (multiply them by themselves)
    //
    // In the closures for both the .filter() and .map() the argument will be a reference, so you'll
    // either need to dereference the argument once in the parameter list like this: `|&x|` or you
    // will need to dereference it each time you use it in the expression like this: `*x`
    v.iter()
        .filter(|x| { *x%2 == 0 })
        .map(|x| { *x * *x })
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 2. Spawn a child thread and have it call `expensive_sum(my_vector)`.  Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the Child thread output in the middle of the main thread's letters
    //
    let handle = thread::spawn(move || {
        expensive_sum(my_vector);
    });

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    // 3. Let's retrieve the value returned by the child thread once it has exited.  Using the
    // `handle` variable you stored the join handle in earlier, call .join() to wait for the thread
    // to exit with a `Result<i32, Err>`.  Get the i32 out of the result and store it in a `sum`
    // variable.  Uncomment the println.  If you did 1a and 1b correctly, the sum should be 20.
    //
    let sum = handle.join().unwrap();
    //println!("The child thread's expensive sum is {}", sum);

    // Time for some fun with threads and channels!  Though there is a primitive type of channel
    // in the std::sync::mpsc module, I recommend always using channels from the crossbeam crate,
    // which is what we will use here.
    //
    // 4. Uncomment the block comment below (Find and remove the `/*` and `*/`).  Examine how the
    // flow of execution works.  Once you understand it, alter the values passed to the `pause_ms()`
    // calls so that both the "Thread B" outputs occur before the "Thread A" outputs.

    
    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(200);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(0);
        tx.send("Thread B: 2").unwrap();
    });
    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }
    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();
    

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.

    let letters = vec!["a", "b", "c", "d", "e", "f"];
    let (my_tx, my_rx) = channel::unbounded();
    let my_rx2 = my_rx.clone();
    
    let r1 = thread::spawn(move || {
        for msg in my_rx2 {
            println!("R1 thread: Received {}", msg);
        }
    });    
    let r2 = thread::spawn(move || {
        for msg in my_rx {
            println!("R2 thread: Received {}", msg);
        }
    });
    

    
    for letter in letters {
        my_tx.send(letter).unwrap();
    }
    drop(my_tx);

    r1.join().unwrap();
    r2.join().unwrap();

    
    println!("Main thread: Exiting.")
}