//Traditional Struct
struct Color {
	red : u8,
	green : u8,
	blue : u8
}

//Tuple Struct 
struct TupleColor(u8, u8, u8);

struct Person{
	first_name : String,
	last_name : String
}

impl Person {
	fn new(first : &str, last : &str) -> Person {
		Person {
			first_name : first.to_string(),
			last_name : last.to_string()
		}
	}

	fn get_full_name(&self) -> String {
		format!("FullName: {} {}", self.first_name, self.last_name)
	}

	fn set_last_name(&mut self, last : &str){
		self.last_name = last.to_string();
	}
}




pub fn do_this(){
	let mut c = Color {
		red : 255,
		green : 0,
		blue : 0
	};

	c.green = 250;

	println!("Color : {} {} {}", c.red, c.blue, c.green);

	let tc = TupleColor(240,250,250);
	println!("Tuple Color : {} {} {}", tc.0, tc.1, tc.2);


	let mut p1 = Person::new("Jason", "Ron");

	println!("Person {} {}", p1.first_name, p1.last_name);
	
	println!("Person Method : {} ", p1.get_full_name());

	p1.set_last_name("Jackson");
	println!("Person GetFullName: {}", p1.get_full_name());
	
}