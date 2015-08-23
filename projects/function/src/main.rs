fn main() {
	let y = 10;
	let y = 100;
    print_sum(5);
	println!("y is {}", y);
	println!("------------------");
	let tuple = (1, 2, 3);
	let x = tuple.0;
	let y = tuple.1;
	let z = tuple.2;
	println!("x = {}", x);
	println!("y = {}", y);
	println!("z = {}", z);
	println!("------------------");
	println!("h(5) = {}",h(5));

}

fn print_sum(x : i32){
	println!("x is {}", x);
}

fn foo(x: i32) -> i32 { x }
let h: fn(i32) -> i32 = foo;
