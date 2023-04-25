pub fn gauss(n_size: usize, b: &mut Vec<f64>, x: &mut Vec<f64>, a: &mut Vec<Vec<f64>>, num_thread: usize){
	
	let pool = rayon::ThreadPoolBuilder::new().num_threads(num_thread).build().unwrap();	
	
	for norm in 0..(n_size-1) {
		let mut at = a.clone();
		let mut bt = b.clone();
		pool.install(move || {
			for row in norm+1..n_size {
				let multiplier = at[row][norm]/at[norm][norm];
				for col in norm..n_size {
					at[row][col] -= at[norm][col] * multiplier;
				}
				bt[row] -= bt[norm] * multiplier;
			}
		});
	}
	
	
	for row in  (0..(n_size)).rev() {
		x[row] = b[row];
		for col in (row..(n_size)).rev() {
			x[row] -= a[row][col] * x[col];
		}
		x[row] /= a[row][row];
	}
}
