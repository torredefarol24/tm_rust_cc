pub fn exec(){
	let x:i32 = 4;
	let y:f64 = 2.543;
	let is_active:bool = true;
	let is_greater:bool = 10>25;
	let face = '\u{1F600}';

	println!("{:?}", (x,y,is_active, is_greater, face));
}