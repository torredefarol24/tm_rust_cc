pub fn run(){
	let mut count = 0;
	
	loop {
		count += 1;
		println!("Number :\t{}", count);
		
		if count == 20 {
			break;
		}
	}

	count = 0;
	while count <=100{
		if count %15 == 0{
			println!("Fizz Buzz");
		} else if count %5 ==0 {
			println!("Fizz");
		} else if count % 3 == 0 {
			println!("Buzz");
		} else {
			println!("{}", count)
		}
		count += 1;
	}

	count = 0;
	for x in 0..100{
		if x % 15 == 0 {
			println!("Fizz Buzz");
		} else if x %5 ==0 {
			println!("Fizz");
		} else if x % 3 == 0 {
			println!("Buzz");
		} else {
			println!("{}", count)
		}
	}

}