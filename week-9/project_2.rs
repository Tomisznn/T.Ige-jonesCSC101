use std::io::Write;
use std::io::{self};

fn main() -> io::Result<()> {
	let name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
	let mat = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
	let dep = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
	let level = vec!["300", "100", "200", "200", "100"];

	for i in 0..name.len(){
		println!("
			Student name: {} 
			Matric number: {} 
			Department: {} 
			Level: {}",name[i],mat[i],dep[i],level[i] );
	}
	


	let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Failed to create file");
	file.write_all("PAU SMIS".as_bytes()).expect("Failed to write");

	for i in 0..name.len(){
		writeln!(file,"
			Student name: {} 
			Matric number: {} 
			Department: {} 
			Level: {}",name[i],mat[i],dep[i],level[i] )?;
	}
	Ok(())




    
}
