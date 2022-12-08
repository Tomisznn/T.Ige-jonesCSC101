use std::io;


fn main() {
    println!("Rust calculator");
    println!("Select number of the equation being used");
    println!("1: Area of Trapezium formula = height/2 * (base1+base2)");
    println!("2: Area of the rhombus formula = ½ × diagonal1 × diagonal");
    println!("3: Area of Parallelogram formula = base x altitude");
    println!("4: Area of Cube formula = 6 x (length of the side)^2 ");
    println!("5: Volume of Cylinder formula = π * radius^2 * height");

    let mut input1 = String::new();

    println!("Input number of the equation: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let a:f32 = input1.trim().parse().expect("Invalid number");

    if a == 1.0
    {
    	trap();
    }
    else if a == 2.0
    {
    	rhom();
    }
    else if a == 3.0
    {
    	para();
    }
    else if a == 4.0
    {
    	cube();
    }
    else if a == 5.0
    {
    	cylin();
    }

}
// Area of Trapezium
fn trap(){
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();

	println!("Input height");
	io::stdin().read_line(&mut input2).expect("Invalid input");
    let h:f32 = input2.trim().parse().expect("Invalid number");

    println!("Input base 1");
	io::stdin().read_line(&mut input3).expect("Invalid input");
    let b:f32 = input3.trim().parse().expect("Invalid number");

    println!("Input base 2");
	io::stdin().read_line(&mut input4).expect("Invalid input");
    let c:f32 = input4.trim().parse().expect("Invalid number");

    let trapezium = (h / 2.0) * (b + c);

    println!("The area of the Trapezium is {} cm^2",trapezium );

}


// Area of rhombus

fn rhom(){
	let mut input5 = String::new();
	let mut input6 = String::new();

	println!("Input diagonal 1: ");
	io::stdin().read_line(&mut input5).expect("Invalid input");
	let d:f32 = input5.trim().parse().expect("Invaid number");

	println!("Input diagonal 2: ");
	io::stdin().read_line(&mut input6).expect("Invalid input");
	let e:f32 = input6.trim().parse().expect("Invaid number");

	let rhombus = 0.5 * d * e;

	println!("The area of the rhombus is {} cm^2",rhombus );
}


// Area of Parallelogram

fn para(){
	let mut input7 = String::new();
	let mut input8 = String::new();

	println!("Input base: ");
	io::stdin().read_line(&mut input7).expect("Invalid input");
	let f:f32 = input7.trim().parse().expect("Invalid number");

	println!("Input altitude: ");
	io::stdin().read_line(&mut input8).expect("Invalid input");
	let g:f32 = input8.trim().parse().expect("Invalid number");

	let parallelogram = f * g;

	println!("The area of the parallelogram is {} cm^2",parallelogram );
}


// Area of cube 

fn cube(){
	let mut input9 = String::new();

	println!("Input length: ");
	io::stdin().read_line(&mut input9).expect("Invalid input");
	let l:f32 = input9.trim().parse().expect("Invalid number");

	let cub = 6.0 * (l * l);

	println!("The area of the cube is {} cm^2",cub );
}


// Volume of Cylinder formula

fn cylin(){
	let mut input10 = String::new();
	let mut input11 = String::new();

	println!("Input radius: ");
	io::stdin().read_line(&mut input10).expect("Invalid input");
	let r:f32 = input10.trim().parse().expect("Invalid number");

	println!("Input height: ");
	io::stdin().read_line(&mut input11).expect("Invalid input");
	let x:f32 = input11.trim().parse().expect("Invalid number");

	let cylinder = (22.0/7.0) * (r*r) * x;

	println!("The volume of the cylinder is {} cm^3",cylinder );

}

