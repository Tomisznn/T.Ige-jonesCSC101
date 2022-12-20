fn main() {
    // make a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}",mountain_heights );

    //how to change the 3rd and 4th element of the tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}",mountain_heights );
}
