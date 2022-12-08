use std::io;


fn main() {
    println!("Siblings program");
    // number of siblings
    let mut input1 = String::new();

    println!("Input number of siblings: ");
    io::stdin().read_line(&mut input1).expect("invalid input");
    let a:i32 = input1.trim().parse().expect("invalid number");

    let mut final_output = String::new();




    for i in 0..a{ 
    	let mut name = String::new();
    	let mut age = String::new();

    	println!("Input first name of sibling: ");
    	io::stdin().read_line(&mut name).expect("Invalid input");
    	
    	println!("Input age of sibling: ");
    	io::stdin().read_line(&mut age).expect("Invalid input");
    	let b:f32 = age.trim().parse().expect("Invalid number");

    	if b > 18.0 {
    		let mut status = String::new();
    		

    		println!("Marital status");
    		println!("\nMarried or single? (If married input 1, if single input 0)");
    		io::stdin().read_line(&mut status).expect("invalid input");
    		let status:f32 = status.trim().parse().expect("invalid number");
    		let mut status_1 = String::new();
    		
    		if status == 1.0{
    			status_1 = "married".to_string();

    			let mut child = String::new();
    			let mut city = String::new();

    			println!("Do they have any offspring? ");
    			io::stdin().read_line(&mut child).expect("Invalid input");

    			println!("What city do they live in?");
    			io::stdin().read_line(&mut city).expect("invalid input");

    			let data = format!("Sibling{},name: {},age: {},status: {},children?: {},city: {}, ,", i+1,name,age,status_1,child,city);
    			final_output.push_str(&data);


    		}

    		if status == 0.0{
    			status_1  = "single".to_string();

    			let mut occupation = String::new();



    			println!("Are you a student or worker?  (if student input 1, if worker input 0)");
    			io::stdin().read_line(&mut occupation).expect("Invalid input");
    			let occupation:f32 = occupation.trim().parse().expect("Invalid input");
    			let mut occupation_1 = String::new();

    			if occupation == 1.0{
    				occupation_1 = "student".to_string();

    				let mut university = String::new();
    				let mut course = String::new();

    				println!("What university do you attend?");
    				io::stdin().read_line(&mut university).expect("Invalid input");

    				println!("What course do you study?");
    				io::stdin().read_line(&mut course).expect("Invalid input");

    				let data = format!("Sibling {},name: {},age: {},status: {},occupation: {},university: {},course: {}, ,", i+1,name,age,status_1,occupation_1,university,course);
    			final_output.push_str(&data);

    			}
    			else if occupation == 0.0{
    				occupation_1 = "worker".to_string();

    				let data = format!("Sibling {},name: {},age: {},status: {},occupation: {}, ,", i+1,name,age,status_1,occupation_1);
    			final_output.push_str(&data);

    			}
    			


    		}
	

    	}

    	if b < 18.0 {
    		let mut waec = String::new();

    		println!("Waec status");
    		println!("Did you write waec? ( If yes input 1, if no input 0.)");
    		io::stdin().read_line(&mut waec).expect("Invalid input");
    		let waec:f32 = waec.trim().parse().expect("invalid number");
    		let mut waec_1 = String::new();

    		if waec == 1.0{
    			waec_1 = "yes".to_string();


    			println!("What school do you attend?");	
    			let mut school = String::new();
    			io::stdin().read_line(&mut school).expect("Invalid input");

    			let data = format!("Sibling {},name: {},age: {},waec: {},school: {}, ,", i+1,name,age,waec_1,school);
    			final_output.push_str(&data);

    		}
    		else if waec == 0.0{
    			waec_1 = "no".to_string();
    			
    			let mut class = String::new();

    			println!("Input current class level:");
    			io::stdin().read_line(&mut class).expect("Invalid input");

    			let data = format!("Sibling {},name: {},age: {},waec: {},class: {}, ,", i+1,name,age,waec_1,class);
    			final_output.push_str(&data);

    		}

    	}


    		 } 

    		 let details = final_output.split(",");

    		// println!("{}",details );
    		println!("\n");
    		println!("----------------------------------------------");

    		 for detail in details{
    		 	println!("{}",detail );
    		 }





    


}

