use std::io;

fn main() {
    
    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input1 = String::new();

    println!("Enter an index value between (0-7)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Invalid input");

    //index is the non negative value which is smaller than the size of the vector

    //getting value at given indexvalue
    let ch:char = v[index];

    print!("{} is the charcter for index [{}]\n",ch, index);

}
