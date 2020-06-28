use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 맞혀봅시다!");
	let mut me = 0;
	let secret_number = rand::thread_rng().gen_range(1,101);
	println!("사용자가 맞춰야 할 숫자 {}",secret_number);
	
	println!("정답이라고 생각하는 숫자를 입력하세요.{}",me);
	me = 10;
	loop{
		let mut guess = String::new();
		
		io::stdin().read_line(&mut guess)
			.expect("입력한 값을 읽지 못했습니다.");
		
		println!("입력한 값: {}",guess);
		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => {
				println!("Input Error {}",me);
				continue;
			},
			
		};
		match guess.cmp(&secret_number){
			Ordering::Less => println!("입력숫자가 작습니다"),
			Ordering::Greater => println!("입력숫자가 많습니다"),
			Ordering::Equal => {
			    println!("정답!");
				break;
			},
		}
	}
}
