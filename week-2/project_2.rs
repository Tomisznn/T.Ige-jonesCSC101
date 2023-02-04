fn main() {
	let toshiba:f64 = 2.00 * 450_000.00;
	let mac:f64 = 1.00 * 1_500_000.00;
	let hp:f64 = 3.00 * 750_000.00;
	let dell:f64 = 3.00 * 2_850_000.00;
	let acer:f64 = 1.00 * 250_000.00;
	let sum:f64 = toshiba + mac + hp + dell + acer;
	let average:f64 = sum /10.00;
	println!("The average of the sales record is {} The sum of the record is {}",average,sum );
}
