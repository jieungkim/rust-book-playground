fn main() {
    // let s = "hello";

    // let s = Stirng::from("hello");
    // This is not mutable 
   
    let mut s = String::from("hello");
    s.push_str(", world!!");

    println!("Hello, world!");
    println!("{}", s);


    let s1 = String::from("hello");
    let s2 = s1; 

    // we cannot access s1 any more
    // println!("{}, world!", s1); <- error
    println!("{}, world!", s2);


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    
}
