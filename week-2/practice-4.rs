fn main() {
	p:f64 = 1000.0;
	r:f64 = 1.0;
	t:f64 = 2.0;

	//simple interest
	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amout is {}", a);
	let si = a - p;
	println!("Simple interest is {}", si );

}