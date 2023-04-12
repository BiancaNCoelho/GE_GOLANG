pub fn gauss(n_size: usize, b: &mut Vec<f64>, x: &mut Vec<f64>, a: &mut Vec<Vec<f64>>){
	let mut multiplier: f64;
	
	for norm in 0..(n_size-1) {
		for row in norm+1..n_size {
			multiplier = a[row][norm]/a[norm][norm];
			for col in norm..n_size {
				a[row][col] -= a[norm][col] * multiplier;
			}
			b[row] -= b[norm] * multiplier;
		}
	}
	
	for row in  (0..(n_size)).rev() {
		x[row] = b[row];
		for col in (row..(n_size)).rev() {
			x[row] -= a[row][col] * x[col];
		}
		x[row] /= a[row][row];
	}
}

