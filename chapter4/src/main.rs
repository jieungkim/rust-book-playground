fn main() {
    
	
	{
		let mut s = String::from("hello");
		s.push_str("hello");
		println!("{} word",s);
	}
	
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("{} {} word",s1,s2);
	let s = String::from("Hello");
	takes_ownership(s);
	//println!("{} word",s);
	let x = 5;
	makes_copy(x);
	
	let s1 = gives_ownership();
	let s2 = String::from("hello");
	let s3 = takes_and_gives_back(s2);
	let (s4, len) = calculate_length(&s3);
	println!("s4 : {} len : {} ",s3,len);
	
	let mut s = String::from("!Hellow");
	change(&mut s);
	println!("s {} ",s);
	let r1 = &s;
	let r2 = &s;
	let r3 = &mut s;
	//println!("s4 : {} len : {} ",r1,r2);
	
	let reference_to_nothing = dangle();
	let word = first_word(&s);
	
	println!("len : {} ",word);
	//s.clear();
	
	//let word = first_word(&s);
	//println!("len : {} ",word);
	let hello = &s[0..3];
	let world = &s[4..8];
	println!(" {}  : {} ",hello,world);
	let slice = &s[0..2];
	println!(" {}  ",slice);
	let slice = &s[..2];
	println!(" {}  ",slice);
	let len = s.len();
	
	let slice = &s[3..len];
	println!(" {}  ",slice);
	let slice = &s[3..];
//	println!(" {}  ",slice);
	
	let slice = &s[0..len];
//	println!(" {}  ",slice);
	let slice = &s[..];
//	println!(" {}  ",slice);
	
	let my_string = String::from("hello world");
	
	println!(" {}  ",my_string);
	
	let word = first_word(&my_string[..]);
	
	println!(" {}  ",word);
	
	let my_string_literal = "hello world";
	
	println!(" {}  ",my_string_literal);
	
	let word = first_word(&my_string_literal[..]);
	
	println!(" {}  ",word);
	
	let word = first_word(my_string_literal);
	
	println!(" {}  ",word);
	
	let a = [1,2,3,4,5]; 
	let slice = &a[0..3];
	
	for (i) in slice.iter() {
		println!(" {}  ",i);	
	}
	
}



fn dangle() -> String{
	let s = String::from("hello");
	s
}

fn change(some_string: &mut String){
	some_string.push_str(", world");
}
fn calculate_length(s:&String)->(&String, usize){
	let length = s.len();
	
	(&s,length)
}

fn takes_ownership(some_thing: String){
	println!("something {}",some_thing);
}

fn makes_copy(some_int: i32){
	println!("{}",some_int);
}

fn takes_and_gives_back(a_string: String) -> String{
	a_string
}
fn gives_ownership() -> String{
	let some_string = String::from("hello");
	some_string
}

fn first_word(s: &str) -> &str {

	let bytes = s.as_bytes();
	
	for(i, &item) in bytes.iter().enumerate(){
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}