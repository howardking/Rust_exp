pub fn add_two(a: i32) -> i32 {
	a + 2
}

#[cfg(test)]
mod test{
	use super::add_two;

	#[test]
	#[should_panic(expected = "assertion failed")]
	fn it_works() {
		assert!(false);
	}

	#[test]
	fn test_add_two(){
		assert_eq!(4, add_two(2));
	}
}
