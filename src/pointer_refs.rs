pub fn run(){
	// Primitive Array
	let arr1 = [1,12,3];
	let arr2 = arr1;


	// Vector
	let vec1 = vec![1,12,3];
	let vec2 = &vec1;
	
	println!("Values {:?}", (arr1, arr2));
	println!("Vec Values {:?}", (&vec1, vec2));
}