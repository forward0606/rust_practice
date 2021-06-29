use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("This is a guessing game!");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	loop{
		println!("please enter a number between 1 to 100.");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		let guess:u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => {
				println!("the data you entered is not a number");
				continue;
			},
		};
		println!("your guess is {}", guess);
		match guess.cmp(&secret_number){
			Ordering::Greater => println!("too big"),
				Ordering::Less => println!("too small"),
				Ordering::Equal => {
					println!("you are win");
					break;
				},
		}
	}
}
