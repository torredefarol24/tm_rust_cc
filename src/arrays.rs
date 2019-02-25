pub fn run(){
	let mut numbers: [i32; 6] = [13,24,1,22,4,35];
	
	//print entire array
	println!("\n Arrays \t{:?}", numbers);

	//print single elem
	println!("\n Array Single Elem \t{}", numbers[3]);

	// Reassign a value
	numbers[3] = 155;
	println!("\n Array \t{:?}", numbers);


	//Print array length & Memory
	println!("Array Length: {}", numbers.len());
	println!("Array Occupies {} bytes", std::mem::size_of_val(&numbers));

	// Get Slice 
}