use std::io;

fn main() {
    // Crate an empty vector "city"

    let mut city : Vec<String> = Vec::new();
    // Print City Vector
    
    let mut input1 = String::new();

    println!("The City vector has element {}",city.len() ); //push new elements into
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("Invalid input");

    for count  in 0..city_num {

    	let mut input2 = String::new();

    	println!("Enter city {}",count+1 );
    	io::stdin().read_line(&mut input2).expect("Failed to read input");
    	let new_city:String = input2.trim().parse().expect("Invalid input");
    	city.push(new_city);
    }

    println!("Your preferred cities are:");
    let mut count=1;
    //loop to iterate elements in vector

    for i in city{
    	println!("{} {}",count, i );
    	count+=1;
    }

    

}
