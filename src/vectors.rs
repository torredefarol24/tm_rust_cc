pub fn run(){
	let mut numbers: Vec<i32> = vec![1, 2, 3, 4,5,6,7,8,9];
	
	numbers[1] = 33;
	
	//Get Vector Slice
	let slice: &[i32] = &numbers[1..4];
	println!("Vector Slice \t{:?}", slice);

	//Add to Vector
	numbers.push(12);
	println!("Pushing to Vector \t{:?}", numbers);

	//Pop from Vector
	numbers.pop();
	println!("Popping from Vector \t{:?}", numbers);

	//Loop through vector values
	for x in numbers.iter(){
		println!("Number: {}", x);
	}

	//Loop & mutate values
	for x in numbers.iter_mut(){
		*x *= 3;
	}

	println!("\nNumbers Vec After Mutation: \t{:?}", numbers);
}