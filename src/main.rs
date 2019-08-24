use std::env;

fn main() {
	for arg in env::args().skip(1) {
		print!("{}", procpart(&arg));
	}
	println!();
}


fn procpart(s: &String) -> String {
	let chars: Vec<char> = s.chars().collect();
	let bytes = chars.chunks(2)
			.map( |chunk| {
				match chunk {
					['0', 'x'] => None,
					_ => parsebyte(chunk)
				}
			})
			.filter_map(|x| x)
			.collect();
	match String::from_utf8(bytes) {
		Ok(s) => s.chars().rev().collect(),
		_ => panic!("Coudn't parse bytes into ASCII"),
	}
}
					
fn parsebyte(byte: &[char]) -> Option<u8> {
	let s = byte.iter().cloned().collect::<String>();
	u8::from_str_radix(&s, 16).ok()
}
				
