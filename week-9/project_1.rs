use std::io::Write;

fn main() {
    let lagers = "Lagers: 33 Export, Desperados, Goldberg, Gulder, Heineken, and Star.\n";
    let stouts = "Stouts: Legend, Turbo King, and Williams.\n";
    let non = "Non-Alcoholic: Maltina, Amstel Malta, Malta Gold, and Fayrouz.\n";

    let mut file = std::fs::File::create("NigerianBreweries.txt").expect("Failed to create");
    file.write_all("This file shows the igh-quality categories of drinks.\n".as_bytes()).expect("Failed to write");
    file.write_all(lagers.as_bytes()).expect("Failed to write");
    file.write_all(stouts.as_bytes()).expect("Failed to write");
    
    file.write_all(non.as_bytes()).expect("Failed to write");

    println!("Data has been saved to a file");
}