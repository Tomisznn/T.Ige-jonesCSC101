fn main() {
	//an array of numbers 
	      //index  0  1  2  3  4
	let numbers = [1, 2, 3, 4, 5];
	println!("Original array = {:?}",numbers );

	//create a slice that will show only 2nd and 3rd element
	let slice1 = &numbers[1..3];
	println!("2nd and 3rd elements sliced = {:?}",slice1 );

	// omit the start index
	let slice2 = &numbers[..3];
	//This means the slice starts from index 0 and goes up to index 3  it will print(1,2,3)
	println!("index 0 to index 3 sliced = {:?}",slice2 );

	//omit the end index
	let slice3 = &numbers[2..];
	//This means the slice starts from index to all the way to the final index (index 5 in this case)
	println!("index 2 to index 5 sliced = {:?}",slice3);

	//omit the start index and he end index
	//reference the whole array
	let slice4 = &numbers[..];
	//This means the slice starts from index 0 and goes up to index 5 
	println!("index 0 to index 5 sliced = {:?}",slice4 );
    
    }