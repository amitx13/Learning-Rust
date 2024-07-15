//Struct
// struct User{
//     username:String,
//     email:String,
// }

// enum Shape{
//     Circle(f32), //variend with associated data (radius)
//     Square(f32), //variend with associated data (side length)
//     Rectangle(f32 , f32) //varient with associated data (width , height )
// }
// fn main() {
    // let x:i8 = 13;
    // print!("x => {}\n",x);

    // let f:f32 = 1.3;
    // print!("f => {}\n",f);

    // //&str and string are different
    // let str1:&str = "apx13";
    // print!("str => {}\n",str1);

    // let str2 = String::from("apx13_");
    // print!("str2 => {}\n",str2);

    // let char = str2.chars().nth(9); //similar to console.log(str[0])

    // match char { //patern matching
    //     Some(c) => {println!("char {}", c)},
    //     None => println!("exceeding total numbers of characters..."),
    // }

    // let str3: &str = "_legend";

    // let combine: String = format!("{}{}",str1,str3);
    // println!("combine => {}", combine);

    //mutable referense
    // let mut name = String::from("apx13");
    // update_string(&mut name);
    // println!("{}",name);

    //Referenses
    // let test = String::from("hello");
    // referense_test(&test);
    // println!("{}", test);

    //struct
    // let user = User{
    //     username:String::from("apx13_"),
    //     email:String::from("amitkvs981@gmail.com"),
    // };
    // println!("{}|{}",user.username,user.email);

    //Enums
    // let circle = Shape::Circle(3.0);
    // let square = Shape::Square(4.0);
    // let rectangle = Shape::Rectangle(4.0, 2.0);

    // println!("circle area = {}",calculate_area(circle));
    // println!("Square area = {}",calculate_area(square));   
    // println!("Rectangle area = {}",calculate_area(rectangle));
// }

// fn calculate_area(shape:Shape) -> f32 {
//     match shape{
//         Shape::Circle(radius)=> 3.14 * radius *radius,
//         Shape::Square(length)=> length*length,
//         Shape::Rectangle(w,h )=> w*h
//     }
// }

// fn update_string(str: &mut String) { //mutable referense
//     str.push_str("_hi")
// }

// fn referense_test(str: &String) { //immutable referense
//     println!("{}", str);
// }
// fn main() {
//     let x = 7;
//     println!("{}",x);
//     let x ="amit";
//     println!("{}",x);
// }

// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..100);
//     loop{
//         println!("Please input your guess.");

//         let mut guess = String::new();
    
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line"); 
    
//         let guess: u32 = match guess.trim().parse(){
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small !!"),
//             Ordering::Greater => println!("Too big !!"),
//             Ordering::Equal => {
//                 println!("You Win !!");
//                 break;
//             }
//         }
//     }
// }

// fn main(){
//     let str = String::from("apx13");
//     let s = &str[0..2];
//     println!("{}",s);
//     println!("{}",str)
// }

// Define the trait with default implementations
// trait Animal {
//     fn make_sound(&self);
    
//     fn breed(&self) -> &str {
//         "Unknown"
//     }

//     fn color(&self) -> &str {
//         "Unknown"
//     }
// }

// // Implement the trait for a Dog with specific implementations for make_sound and breed
// struct Dog;

// impl Animal for Dog {
//     fn make_sound(&self) {
//         println!("Woof!");
//     }

//     fn breed(&self) -> &str {
//         "Golden Retriever"
//     }
// }

// // Implement the trait for a Cat with specific implementations for make_sound and color
// struct Cat;

// impl Animal for Cat {
//     fn make_sound(&self) {
//         println!("Meow!");
//     }

//     fn color(&self) -> &str {
//         "Black"
//     }
// }

// // Implement the trait for a Cow using default implementations for breed and color
// struct Cow;

// impl Animal for Cow {
//     fn make_sound(&self) {
//         println!("Moo!");
//     }
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;
//     let cow = Cow;

//     // Call the make_sound method on each animal
//     dog.make_sound();
//     println!("Breed: {}, Color: {}", dog.breed(), dog.color());

//     cat.make_sound();
//     println!("Breed: {}, Color: {}", cat.breed(), cat.color());

//     cow.make_sound();
//     println!("Breed: {}, Color: {}", cow.breed(), cow.color());
// }

//Full Example with Multiple Traits
// trait Animal {
//     fn make_sound(&self);
// }

// trait Named {
//     fn name(&self) -> &str;
// }

// struct Dog;

// impl Animal for Dog {
//     fn make_sound(&self) {
//         println!("Woof!");
//     }
// }

// impl Named for Dog {
//     fn name(&self) -> &str {
//         "Dog"
//     }
// }

// struct Cat;

// impl Animal for Cat {
//     fn make_sound(&self) {
//         println!("Meow!");
//     }
// }

// impl Named for Cat {
//     fn name(&self) -> &str {
//         "Cat"
//     }
// }

// // Function using trait bounds
// fn describe_named_animal<T>(animal: &T)
// where
//     T: Animal + Named,
// {
//     animal.make_sound();
//     println!("Name: {}", animal.name());
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;

//     describe_named_animal(&dog);
//     describe_named_animal(&cat);
// }

//Generic Lifetimes Annotations:
// fn main(){
//     let s1 = String::from("Hey there!");
//     let s2 = String::from("apx13");

//     let res = longest_string(s1.as_str() , s2.as_str());
//     println!("the longest str is : {}",res);
// }

// fn longest_string<'a>(str1:&'a str , str2:&'a str) -> & 'a str {
//     if str1.len() > str2.len(){
//         str1
//     }else{
//         str2
//     }
    
// }

// fn main(){
//     let s1 = String::from("Hey there!");
//     {
//     let s2 = String::from("apx13");
//     let res = longest_string(s1.as_str() , s2.as_str()); //now the lifetime of the res will be similar of s2 acc to generic lifetime annotations
//     println!("the longest str is : {}",res);
//     }
// }

// fn longest_string<'a>(str1:&'a str , str2:&'a str) -> &'a str {
//     if str1.len() > str2.len(){
//         str1
//     }else{
//         str2
//     }
    
// }

//WAP to take two numbers as input and perform calculations and print the output:
// use std::{error::Error, io};
// fn main(){
//     println!("Please Enter two numbers:");
//     let num1 = take_ins();
//     let num2 = take_ins();
//     println!("What operation do you wanna perform +, -, *, /, %");
//     let sign = sign_ins();
//     match perform_operations(&num1, &num2, sign) {
//         Ok(result) => println!("Result of {} {} {}: {}", num1, sign, num2, result),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn take_ins()-> usize{
//     let mut num = String::new();
//     io::stdin().read_line(&mut num).expect("not a valid String");
//     num.trim().parse().expect("cannot be converted into Integer")
// }

// fn sign_ins()-> char{
//     let mut sign = String::new();
//     io::stdin().read_line(&mut sign).expect("not a valid String");
//     sign.trim().chars().next().expect("cannot be converted into char")

// }


// fn perform_operations(num1:&usize, num2:&usize , sign:char) -> Result<usize, Box<dyn Error>> {
//     match sign {
//         '+' => Ok(num1 + num2),
//         '-' => Ok(num1 - num2),
//         '*' => Ok(num1 * num2),
//         '/' => {
//             if *num2 != 0 {
//                 Ok(num1 / num2)
//             } else {
//                 Err("Division by zero is not allowed".into())
//             }
//         }
//         '%' => {
//             if *num2 != 0 {
//                 Ok(num1 % num2)
//             } else {
//                 Err("Modulo by zero is not allowed".into())
//             }
//         }
//         _ => Err("Invalid operation sign".into()),
//     }
// }

//WAP for febonacci series
use std::{error::Error, io};
fn main(){
    println!("Enter a number of terms in the febonacci series");
    let num = input_num_of_term();
    match num {
        Ok(num) => println!("Entered n'th term is : {}",num),
        Err(smt) => println!("{}",smt)
    }
}

fn input_num_of_term() -> Result<usize,Box<dyn Error>>{
    let mut num = String::new();
    io::stdin().read_line(& mut num)?;
    let trimmed_number = num.trim();
    match trimmed_number.parse(){
        Ok(num) => Ok(num),
        Err(..) => Err("cannot be parsed into num".into())
    }
} 