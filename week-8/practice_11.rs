fn main() {
    //mutable array
              //index   0      1        2        3
    let mut colours = ["red","green","yellow","white"];

    println!("Original array = {:?}",colours );

    //mutable slice
    let sliced_colors = &mut colours[1..3];

    println!("First slice = {:?}",sliced_colors );

    // change the value of the original slice at the first index
    //Because you have values of the 'sliced colors' to be green,yellow the index of green
    //will now be 0 and the index of yellow to be 1

    //in the line below this you change the value of index 1 which is now 'yellow' to "purple"
    //This will print (green,purple)
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}",sliced_colors );
}
