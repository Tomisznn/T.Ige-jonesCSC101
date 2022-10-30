fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let t:f64 = 5.0;
	let a = p * (1.00 + (r / 100.00)).powf(t);
	let ci = a - p;
	println!("Compound interest is {}",ci );


}