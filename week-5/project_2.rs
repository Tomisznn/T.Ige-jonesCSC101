use std::io;

fn main() {

	let mut input1 = String::new();
	let mut input2 = String::new();

	println!("How many years of experience do you have?");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let enter:f32 = input1.trim().parse().expect("Not a valid answer");

	println!("How old are you?");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let age:f32 = input2.trim().parse().expect("Not a valid number");

	//assuming the number of years you have to work to be experienced is 10 

	if enter >= 10.0 && age >= 40.0
	{
		println!("Your incentive is N1,560,000");
	}
	else if enter >= 10.0 && age >= 30.0 && age <= 40.0
	{
		println!("Your incentive is N1,480,000");
	}
	else if enter >= 10.0 && age > 28.0
	{
		println!("Your incentive is N1,300,000");
	}
	else if enter < 10.0 
	{
		println!("Your incentive is N100,000");
	}


    
}
