fn main() {
	let mut fibonacci_values = vec![0, 1];
	let mut i = 2;
	loop {
		let n = fibonacci_values[i - 2] + fibonacci_values[i - 1];
		if n > 4_000_000 {
			break;
		}
		fibonacci_values.push(n);
		i += 1;
	}
	let mut sum = 0;
	for value in fibonacci_values {
		if value % 2 == 0 {
			sum += value;
		}
	}
	println!("{}", sum);
}