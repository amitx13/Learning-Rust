//Wap to take a num as a Input and return True and False if num is even or odd

// use std::io;

// fn main(){
//     println!("Enter a number to check if it is even or not");
//     let mut num = String::new();

//     io::stdin()
//     .read_line(&mut num)
//     .expect("Failed to take Input try again!!");

//     let num: i32 = num.trim().parse().expect("Please enter a valid number");

//     if num % 2 == 0  {
//         println!("{} is even",num);
//     } else {
//         println!("{} is odd",num);
//     }
// }

// WAF fib that finds the fibbonacci of a num it takes as an input

// use std::io;

// fn find_fibbonacci(x:u32) -> u32{
//     if x == 0 {
//         0
//     } else if x == 1 {
//         1
//     } else {
//         find_fibbonacci(x - 1) + find_fibbonacci(x - 2)
//     }
// }
// fn main(){
//     let mut num = String::new();

//     println!("Enter a num to find the fibbonacci of a num");

//     io::stdin().read_line(&mut num).expect("Failed to take Input!!");

//     let num:u32 = num.trim().parse().expect("Please Enter a valid num");

//     println!("The {}th Fibonacci number is: {}", num, find_fibbonacci(num));
// }


//WAF that takes input as string and return it's length

// use std::io;

// fn get_string_length(str:String)-> usize{
//     // let mut count:u32 = 0;
//     // for i in str.chars(){
//     //     if i == ' '{
//     //         break;
//     //     }
//     //     count+= 1
//     // }
//     // return count

//     //or just use count 

//     str.chars().count()
// }
// fn main(){
//     let mut str = String::new();

//     println!("Enter a string to find it's length");

//     io::stdin().read_line(&mut str).expect("something went wrong!");

//     println!("Length of the entered string is : {}",get_string_length(str.trim().to_string()));
// }


//struct 
// struct Rect {
//     height: u32,
//     width: u32
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }
// }
// fn main(){
//     let rect = Rect {
//         height:10,
//         width:5
//     };

//     println!("The are of given rectangle is : {}",rect.area());
// }

// Enum
// enum Shape {
//     Rectangle(f64, f64), // variants with associated data 
//     Circle(f64)
// }

// fn main() {
//     let rect = Shape::Rectangle(8.0, 4.0);
//     match calculate_area(rect) {
//         Some(area) => println!("Area of rectangle is : {}", area),
//         None => println!("Failed to calculate area of rectangle"),
//     }

//     let circle = Shape::Circle(5.0);
//     match calculate_area(circle) {
//         Some(area) => println!("Area of circle is : {}", area),
//         None => println!("Failed to calculate area of circle"),
//     }
// }

// fn calculate_area(shape: Shape) -> Option<f64> {
//     match shape {
//         Shape::Rectangle(a, b) => Some(a * b),
//         Shape::Circle(r) => Some(3.14 * r * r),
//     }
// }

//WAP to read content of a file 

// use std::fs;

// fn main(){
//     let content = fs::read_to_string("src/note.txt");

//     match content {
//         Ok(c) => println!("{}", c),
//         Err(e) => println!("Error reading file: {}", e),
//     }
// }

//WAP to show borrowing in rust
// fn main(){
//     let str1 = String::from("apx");
//     print_str(&str1);
//     println!("{}",str1)
// }

// fn print_str(str:&String){
//     println!("{}",str);
// }

//Vector:

// fn main(){
//     let mut vec = vec![1,2,3,4,5,6,7,8,9,10];

//     return_even_vec(&mut vec);

//     println!("{:?}",vec);
// }

// // The &i in the for loop is a shorthand that simplifies dereferencing the reference directly in the loop's body.
// fn return_even_vec(vec:&mut Vec<i32>){
    
//     let mut even_vec = Vec::new();
//     for i in vec.iter(){
//         if *i % 2 == 0 {
//             even_vec.push(*i);
//         }
//     }
//     *vec = even_vec
// }


// fn main(){
//     //Array are primitives in rust so they are stored in stack and not in heap so that's why there is no error
//     //as array is stored in stack and it's value is copied to arr2
//     let arr1 = [1,2,3];
//     let arr2 = arr1;
//     println!("{:?}",(arr1,arr2));
// }

//Hashmap

// use std::collections::HashMap;
// //insert
// //get
// //remove
// //clear
// fn main(){
//     let mut hash = HashMap::new();
//     hash.insert("apx13", 13);
//     hash.insert("apx14", 14);

//     println!("{:?}",hash);

//     let first_user = hash.get("apx13");
//     match first_user {
//         Some(_) => {
//             hash.remove("apx13");
//         },
//         None => println!("No value found"),
//     }

//     println!("{:?}",hash);

// }

//ITERATORS:
//.iter()
//.iter_mut()
//.into_iter()

// fn main() {
//     let vec = vec![1, 2, 3, 4, 5];
//     let iterator = vec.iter();
//     println!("{:?}", iterator);
// }

// fn main(){
//     let  vec = vec![1,2,3,4,5];
//     // let mut iterator = vec.iter();
//     let mut iterator = vec.into_iter();


//     while let Some(val) = iterator.next(){
//         println!("{}",val);
//     }

//     // for i in iterator {
//     //     println!("{}",i);
//     // }
// }

//Ex - consuming adaptors
// fn main() {
//     let vec = vec![1,2,3];
//     let iterator = vec.iter();
//     let total:i32 = iterator.sum();

//     println!("the total of vec: {}",total);

//     //let total2:i32 = iterator.sum(); //can't be called again here becz calling the sum earlier uses up the iterator.
// }

//Ex - Iterator adaptors
// fn main() {
//     let vec = vec![1,2,3];
//     let iter = vec.iter();
//     let iter_2 = iter.map(|x| *x + 1);

//     // for i in iter_2 {
//     //     println!("{}",i);
//     // }

//     let iter_3 = iter_2.filter(|x| *x % 2 == 0);

//     for j in iter_3 {
//         println!("{}",j);
//     }
// }

//WAP to first find all odd values and then double each value and create a new vector 

// fn main(){
//     let vec = vec![1,2,3,4,5,6,7,8];
//     let val = vec.iter().filter(|x| *x % 2 != 0 ).map(|x| x * 2);
//     // for i in val {
//     //     println!("{}",i)
//     // }

//     //or 

//     let new_vec:Vec<i32> = val.collect();
//     println!("{:?}",new_vec);
// }

// fn main(){
//     let word1 = String::from("Hello world");
//     let first_word = &word1[0..5];
//     println!("{}",first_word)
// }


//Lifetimes: how output ka lifetime is associated to input ka lifetime.

// fn main(){
//     let mut ans1 = String::from("apx");
//     {
//         ans1 = String::from("13");
//     }
//     println!("{}",ans1)
// }


// fn greatest <'a> (a: & 'a str , b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }
// fn main(){
//     let var1 = String::from("apx"); 
//     let var2 = String::from("13");
//     let ans = greatest(&var1, &var2);
//     println!("greatest string is: {}",ans);
// }

//Lifetime in Struct

// struct User <'a>{
//     name: &'a str
// }

// fn main(){
//     let first_user = String::from("amit");
//     let user1 = User { name: &first_user };
//     println!(" The name of the first user is: {}::{}",user1.name,first_user);
// }

//Multithreading:

// use std::thread;
// use std::time::Duration;

// fn main(){
//     thread::spawn(|| {
//         for i in 1..1000000000 {
//             println!("spawn Thread: {}",i);
//             // thread::sleep(Duration::from_millis(1));
//         }
//     });/* .join().unwrap(); */ //join is used to await for the spawned thread to finish before running the iteration on the main thread

//     for i in 1..1000000000 {
//         println!("Main Thread: {}",i);
//     }

// }

// use std::thread;


// fn main(){
//     let vec = vec![1,2,3,4,5,6,7,8,9,10];
//     thread::spawn(move || { 
//         /*  move is used to move the ownership of the variable to the spawned thread because vec could go out of scope
//             before the spawned thread finishes executing */
//         println!("{:?}",vec);
//     }).join().unwrap();
// }

// mpsc channel

use std::{sync::mpsc, thread};

fn main(){
    let (tx ,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("apx");
        tx.send(val).unwrap();
    });

    let val:String = rx.recv().unwrap();
    println!("Received value: {}",val);
}