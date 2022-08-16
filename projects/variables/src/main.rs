// const CONST_NUMBER: u32 = 60 * 60 * 3;  constant
// use std::io;

fn main() {
    // let mut x = 5;
    // println!("{x}");
    // x = 6;
    // println!("{x}");

    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let guess: i64 = "42".parse().expect("Not a number!");
    // let t = true;

    // let f: bool = false; // with explicit type annotation
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat: char = '😻';

    // println!("{c} {z} {heart_eyed_cat}");
    // let mut x = 5; 
    // x = 20;
    // println!("{x}");
    // let mut x = 10; // shadowing x
    // println!("{0}", x+1); 
    // let CONST_NUMBER = 10000;
    // let CONST_NUMBER: u32 = 10001;
    // const CONST_NUMBER: u32 = 10000;
    // // let CONST_NUMBER: u32 = 10001;
    // println!("{}",CONST_NUMBER);

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("{x}");
    // }
    // println!("{}", x+1);
    // println!("{}", x);

    // let spaces = "    ";
    // let spaces = spaces.len(); // shadwoing
    // println!("{}",spaces);

    // let x = 2.0; // f64 default
    // let y: f32 = 3.0; // f32 
    // println!("{}{}",x,y);


    // let q1: f32 = 22.0 / 7.0;
    // let q2:f32= 22 / 7;
    // println!("{q1} {q2}");

    // println!("number 1");
    // let mut n1: String = String::new();
    // io::stdin() // io::stdin()  
    //     .read_line(&mut n1)
    //     .expect("Failed to read line");
    // // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing guess
    // let n1: f64 = n1.trim().parse().expect("Please type a number!");
    
    // println!("number 2");
    // let mut n2: String = String::new();
    // io::stdin() // io::stdin()  
    //     .read_line(&mut n2)
    //     .expect("Failed to read line");
    // // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing guess
    // let n2: f64 = n2.trim().parse().expect("Please type a number!");

    // let m = n1 / n2;
    // println!("{m}");

    // let t = true;
    // let f: bool = false;

    // println!("{t}{f}")

    // let mut t: String = String::new();
    // t = "EHELLO WO".to_string();
    // println!("{t} {}",t.len());

    // let tup: (i32, f32, String, char) = (2, 2.1, "Hello world".into(), 'C');
    // println!("{} {} {} {}",tup.0,tup.1,tup.2,tup.3);
    // let (x,y,z,a) = tup;
    // println!("{x} {y} {z} {a}");

    // let x: String;
    // x = "hahaha".into();
    // println!("{x}");
    // let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5];
    // a = [1, 2, 3, 4, 5];
    // let b: [3.1; 5]; // [3.1, 3.1, 3.1, 3.1, 3.1]

    // let a: [i32; 5];
    // a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

    
    // fn1(10.into(), '1');
    // fn fn2() {
    //     println!("haha");
    // }
    // fn2()

    let x = plus_one(5);

    // println!("The value of x is: {x}");

}

fn plus_one(x: i32) {
    x + 1;
    ()
}

// fn fn1(x: f64, y: char) {
//     println!("{x} {y}");
// }
