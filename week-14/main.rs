use std::io;
use std::io::Read;

fn main() {
    println!("Welcome to the Database Management Application by Globacom");

    println!("\nIf you are an administrator, input 1.");
    println!("If you are a project manager, input 2.");
    println!("If you are an employee, input 3.");
    println!("If you are a customer, input 4.");
    println!("If you are a vendor, input 5.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid response");
    let code:f32 = input.trim().parse().expect("Invalid response");

    if code == 1.0{

    	department();
    	
    }

    if code == 2.0{

    	project();
    	
    }

    if code == 3.0{
    	
    	staff();
    }

    if code == 4.0{

    	customer();
    	
    }

    if code == 5.0{

    	dataplan();
    	
    }
}

fn department(){
	let mut file = std::fs::File::open("department_tb.sql").unwrap();
    	let mut contents = String::new();
    	file.read_to_string(&mut contents).unwrap();
    	print!("{}",contents );

}

fn project(){

	let mut file = std::fs::File::open("project_tb.sql").unwrap();
    	let mut contents = String::new();
    	file.read_to_string(&mut contents).unwrap();
    	print!("{}",contents );

}

fn staff(){
	let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    	let mut contents = String::new();
    	file.read_to_string(&mut contents).unwrap();
    	print!("{}",contents );

}

fn customer(){

	let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    	let mut contents = String::new();
    	file.read_to_string(&mut contents).unwrap();
    	print!("{}",contents );

}

fn dataplan(){

	let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    	let mut contents = String::new();
    	file.read_to_string(&mut contents).unwrap();
    	print!("{}",contents );

}
