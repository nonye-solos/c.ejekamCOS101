fn main(){
	let p:f64 = 210000;
	let t:f64 = 3;
	let r:f64 = 5;

	//compound interest
	let a = p *(1.0 - (r/100.0))powif t;
	println!("Amount is {}", a);
	let ci = a - p
}