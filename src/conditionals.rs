pub fn perform(){
	let age: u8 = 18;
	let check_id : bool = true;

	if age >= 21 && check_id {
		println!("Man : What biscuit would you like to eat?");
	} else if age < 21 && check_id {
		println!("Man : You are too young!");
	} else if age > 15 || check_id{
		println!("Wow")
	} else {
		println!("Cool")
	}

	//let shorthand 
	let is_of_age = if age >= 21 { true } else { false };
	println!("Is of Age : {}", is_of_age);
}