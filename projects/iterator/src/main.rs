fn main() {
	for x in 0..10{
		println!("{}", x);
	}

	let mut range = 0 .. 10;
	loop{
		match range.next(){
			Some(x) => {
				println!("{}", x);
			},
			None => {break}
		}
	}

	println!("------------------");

	let nums = vec![1,2,3];

	for num in &nums {
		println!("{}", *num);
	}
	println!("------------------");

	let sum = (1..101).fold(0, |sum, x| sum + x);
	println!("1+...+100 = {}", sum);
	println!("------------------");

	let one_to_onehundred = (1..101).collect::<Vec<i32>>();
	
	for x in one_to_onehundred{
	//	println!("{}", x);
	}

	let greater_than_forty_two = (1..101).find(|x| *x > 42);

	match greater_than_forty_two {
		Some(_) => println!("We got some numbers!"),
		None => println!("No numbers found:("),
	}

	
	println!("------------------");
	//(1..100).map(|x| x + 1, println!("{}", x));
	(1..100).map(|x| println!("{}", x));
	//(1..100).map(|x| x + 1);

	println!("------------------");

	for i in (1..).take(5){
		println!("{}", i);
	}
	
	println!("------------------");

	for i in (1..100).filter(|&x| x % 2 == 0){
		println!("{}", i);
	}
	
	println!("------------------");

	let six_div = (1..1000).filter(|&x| x % 2 == 0).filter(|&x| x % 3 == 0).take(5).collect::<Vec<i32>>();
	for i in six_div{
		println!("{}", i);
	}
}
