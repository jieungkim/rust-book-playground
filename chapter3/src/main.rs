
fn five() -> i32 {
	5
}
fn main() {
	/*
	let mut x = 5;
	println!("x의 값 {}",x);
	x = 6;
	println!("x의 값 {}",x);
	
	
	let x = 5;
	println!("x의 값 {}",x);
	let x = x + 1;
	println!("x의 값 {}",x);
	let x = x + 2;
	println!("x의 값 {}",x);
	
	
	let mut spaces = "    ";
	println!("spaces의 값 {} 11",spaces);
	let spaces = spaces.len();
	
	println!("spaces의 값 {}",spaces);
	
	
	let guess: u32 = "42".parse().expect("숫자가 아닙니다.");
	
	println!("guess의 값 {}",guess);

	
	let x = 2.0;
	let y: f32 = 3.0;
	
	let sum = 5 +10;
	let diff = 95.5 - 4.2;
	let product = 4 * 30;
	let quotient = 56.7/32.2;
	let remainder = 43%5;
	
	let t = true;
	let f : bool = false;
	
	let c = 'z';
	let z = 'Z';	
	let heart_eyed_cat = '철';
	
	let tup : (i32, f64, u8) = (500,6.4,1);
	let tup = (500,6.4,1);
	let (x,y,z) = tup;
	println!("y의 값: {}",y);

	let x : (i32, f64, u8) = (500,6.4,1);
	let five_hundred = x.0;
	let six_point_four = x.1;
	let one = x.2;
	
	let a = [1,2,3,4,5];
	
	let a: [i32;5] = [1,2,3,4,5];
	let a =  [3;5];
	
	for element in a.iter(){
		println!("{}",element);
	}
	
	println!("{}",a[5]);

	another_function(5,6);
	println!("{}",five());
	println!("{}",plus_one(5));
	
	let number = 3;
	
	let condition = true;
	let number = if condition {
		4
	
	}else {
		6
	};
	if number < 5 {
		println!("{}",five());
	
	} else {
		println!("{}",plus_one(5));
	
	}
	
	
	
	let number = if condition {
		4
	
	}else {
		"six"
	};
		*/
	let mut counter = 0;
	let result = loop{
		counter += 1;
		if counter == 10{
			break counter * 2;
		}
	
	};
	
	println!("{}", result);
	
	while counter != 0 {
		println!("{}", counter);
		counter = counter - 1;
	}
	
	for number in (1..10) {
		println!("{}",number);
	}
	
}

fn another_function(x:i32,y:i32){
	let x = 5;
	
	let y = {
		let x = 3; 
		x + 1
		};

	println!("Another Func {}, {}",x,y);
}

fn plus_one(x: i32) -> i32 {
  x + 1
}