fn main() {
	let x = 5;
	#let y = 6;
	#println!("{}", x + y);

	#let x = 5;
	let y = 6;
	#println!("{}", x + y);

	#let x = 5;
	#let y = 6;
	println!("{}", x + y);
}
