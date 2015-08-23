fn foo(x: &i32){
	let y = 10;
	let z = &y;

	println!("this is in foo x = {}", x);

	baz(z);
	bar(x, z);
}

fn bar(a: &i32, b: &i32){
	let c = 5;
	let d = Box::new(5);
	let e = &d;

	println!("This is in bar a = {}, b = {}", a, b);

	baz(e);
}

fn baz(f: &i32){
	let g = 100;
}

fn main() {
	let h = 3;
	let i = Box::new(20);
	let j = &h;
	println!("This is in main j = {}", j);
	foo(j);
}
