pub fn now(){
	greeting("Hey", "John");
	let result = add(14,15);
	println!("Sum is {}", result);

	//Closure
	let add_nums = |num1: i32, num2: i32| num1 + num2;
	println!("New Sum is {}", add_nums(12,15));

}

fn greeting(greet: &str, name: &str){
	println!("{} {}, nice to meet you", greet, name);
}

fn add(num1:i32, num2: i32) -> i32{
	num1 + num2
}