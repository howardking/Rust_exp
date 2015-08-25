fn main() {
	for (i, j) in (5..10).enumerate(){
		println!("i = {} and j = {}", i, j);
	}
    println!("Hello, world!");
	let mut x = 5;
	let mut done = false;

	while !done {
		x += x -3;

		println!("{}", x);

		if x % 5 == 0 {
			done = true;
		}
	}

	println!("--------------------");

	for x in 0u32..10 {
		if x % 2 == 0 {
			continue;
		}

		println!("{}", x);
	}

	println!("--------------------");

	let v = vec![1, 2, 3];
	let v2 = &v;
	println!("v[0] is : {}", v[0]);

	println!("--------------------");

	take(v2);

	
	println!("v[0] is : {}", v[0]);

	println!("--------------------");

	let a = 5;
	let _y = double(a);
	println!("{}",a);
		
	println!("--------------------");

	let a = true;
	let _y = change_truth(a);
	println!("{}", a);

	println!("--------------------");

	let mut x = 5;
	{
		let y = &mut x;
		*y += 1;
	}
	println!("{}", x);

}

fn take(v: &Vec<i32>){
	
}

fn double(x : i32) -> i32 {
	 x * 2
}

fn change_truth(x : bool) -> bool {
	!x
}
