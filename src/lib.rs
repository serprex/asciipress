use std::char::from_u32_unchecked;

unsafe fn adjust(x: u32) -> char {
	from_u32_unchecked(if x >= 0xd800 {
		x + 0xd2800
	} else {
		x
	})
}

pub fn encode(x: &[u8]) -> String
{
	let mut ret = String::new();
	for x in x.chunks(5) {
		let ch1 = match x.len() {
			1 => x[0] as u32,
			2 => x[0] as u32|(x[1] as u32)<<7,
			_ => x[0] as u32|(x[1] as u32)<<7|(x[2] as u32&15)<<14,
		};
		ret.push(unsafe { adjust(ch1) });
		if let Some(ch2) = match x.len() {
			4 => Some(((x[2] as u32)>>4)|(x[3] as u32)<<4),
			5 => Some(((x[2] as u32)>>4)|(x[3] as u32|(x[4] as u32)<<7)<<4),
			_ => None,
		} {
			ret.push(unsafe { adjust(ch2) });
		}
	}
	ret
}

fn unadjust(x: char) -> u32 {
	let x = x as u32;
	if x >= 0xd800 {
		x - 0xd2800
	} else {
		x
	}
}

pub fn decode(x: &str) -> Vec<u8>
{
	let mut ret = Vec::new();
	let mut chs = x.chars();
	while let Some(ch1) = chs.next() {
		let u1 = unadjust(ch1);
		ret.push((u1&127) as u8);
		ret.push((u1>>7&127) as u8);
		if let Some(ch2) = chs.next() {
			let u2 = unadjust(ch2);
			ret.push((u1>>14&127|(u2&15)<<4) as u8);
			ret.push(((u2>>4)&127) as u8);
			ret.push(((u2>>11)&127) as u8);
		}
	}
	while let Some(&0) = ret.last() {
		ret.pop();
	}
	ret
}

pub fn encode_str(x: &str) -> String
{
	encode(x.as_bytes())
}

pub fn decode_str(x: &str) -> String
{
	unsafe { String::from_utf8_unchecked(decode(x)) }
}

#[cfg(test)]
mod tests {
	use super::*;
	const TESTR: &'static str = "asdf hello";
	#[test]
	fn test() {
		println!("{:?}", TESTR.as_bytes());
		let enc = encode(TESTR.as_bytes());
		assert!(enc.chars().count() < TESTR.chars().count());
		let dec = decode(&enc);
		println!("{:?}", dec);
		assert!(dec == TESTR.as_bytes());
	}
	#[test]
	fn test_str() {
		println!("{}", TESTR);
		let enc = encode_str(TESTR);
		assert!(enc.chars().count() < TESTR.chars().count());
		let dec = decode_str(&enc);
		println!("{}", dec);
		assert!(dec == TESTR);
	}
}
