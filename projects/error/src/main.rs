use std::fs::File;
use std::io;
use std::io::prelude::*;
#[derive(Debug)]
enum Version { Version1, Version2 }

#[derive(Debug)]
enum ParseError { InvalidHeaderLength, InvalidVersion }

fn parse_version(header: &[u8]) -> Result<Version, ParseError> {
	if header.len() < 1 {
		return Err(ParseError::InvalidHeaderLength);
	}

	match header[0] {
		1 => Ok(Version::Version1),
		2 => Ok(Version::Version2),
		_ => Err(ParseError::InvalidVersion)
	}
}

struct Info{
	name:String,
	age:i32,
	rating:i32,
}

fn write_info(info: &Info) -> io::Result<()>{
	let mut file = File::create("my_best_friends.txt").unwrap();

	if let Err(e) = writeln!(&mut file, "name: {}", info.name){
		return Err(e)
	}

	if let Err(e) = writeln!(&mut file, "age: {}", info.age){
		return Err(e)
	}

	if let Err(e) = writeln!(&mut file, "rating: {}", info.rating){
		return Err(e)
	}

	return Ok(());
}

fn main() {

	let version = parse_version(&[1, 2, 3, 4]);
	match version{
		Ok(v) => {
			println!("working with version:{:?}", v);
		}
		Err(e) => {
			println!("error parsing header:{:?}", e);
		}
	}
	//panic!("boom");

	println!("---------------------------------");
	

}
