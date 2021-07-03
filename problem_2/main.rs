fn main() {
	let mut sum = 0;
	let mut second_to_last = 0;
	let mut last = 1;
	loop {
		last = second_to_last + last;
		second_to_last = last - second_to_last;
		if last > 4_000_000 {
			break;
		}
		if last % 2 == 0 {
			sum += last;
		}
	}
	println!("{}", sum);
}