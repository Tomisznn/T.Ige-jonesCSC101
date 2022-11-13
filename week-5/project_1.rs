use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("The value of a: ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:f32 = input1.trim().parse().expect("Not a valid number");

	println!("The value of b: ");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:f32 = input2.trim().parse().expect("Not a valid number");

	println!("The value of c:");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let c:f32 = input3.trim().parse().expect("Not a valid number");

	let mut det = (b * b) - (4.0 * a * c);

	if det > 0.0 
	{
		println!("The roots will be:");
		let roota = (-b + det.sqrt()) / (2.0 * a);
		let rootb = (-b - det.sqrt()) / (2.0 * a);
		println!("Root A = {}" ,roota);
		println!("Root B = {}",rootb);
	}
	else if det == 0.0 
	{
		println!("There is exactly one real root which is {}",det );
	}
	else if det < 0.00 
	{
		println!("There are no real roots but the imaginary roots are: ");
		let root = -b / (2.0 * a);
		det = det.abs();
		let imag = det.sqrt() / ( 2.0 * a);
		println!("Root A = {} +i {}",root,imag );
		println!("Root B = {} -i {}",root,imag);
	}
    
}
