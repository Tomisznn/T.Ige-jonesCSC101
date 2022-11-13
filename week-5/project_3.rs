use std::io;


fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();

	println!("Menu & Price:");
	println!("Poundo Yam/Edinkaiko Soup  -  N3,200");
	println!("Fried Rice & Chicken  -  N3,000");
	println!("Amala & Ewedu Soup. -  N2,500");
	println!("Eba & Egusi Soup. -  N2,000");
	println!("White Rice & Stew. -  2,500");

    println!("What is the price of first order?");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:f32 = input1.trim().parse().expect("Not a valid order");

    println!("quantity of order");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:f32 = input2.trim().parse().expect("Not a valid order");

	println!("any other orders? if none input 0");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let c:f32 = input3.trim().parse().expect("Not a valid order");

	println!("quantity of second order");
	io::stdin().read_line(&mut input4).expect("Not a valid string");
	let d:f32 = input4.trim().parse().expect("Not a valid order");


	let s = (a * b) + (c * d);

	if s > 10000.00 
	{
		let d = s * (5.0 / 100.0);
		let e = s - d;
		println!("The total price = {}",e );
	}
	else if s <= 10000.00
	{
		println!("The total price = {}",s );
	}





    
}
