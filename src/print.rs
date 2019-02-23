pub fn run() {
	//Printing
	println!("Hey There.");
	println!("Number: {}", 1);

	// Formatting
	println!("Message: {}", "Wow");
	println!("{} comes from {}", "TFQ", "DHK");

	//Positional Args
	println!("{0} comes from {1} and {0} also likes to {2}", "TFQ", "DHK", "eat");

	//Named Args
	println!("{name} likes to eat {food}", name = "TFQ", food="bread");

	// PLaceholder for debug
	println!("{:?}", (12, true, "wow"));
}