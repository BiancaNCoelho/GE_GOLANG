use rayon::prelude::*;

pub fn gauss(n_size: usize, b: &mut Vec<f64>, x: &mut Vec<f64>, a: &mut Vec<Vec<f64>>){
	
	for norm in 0..(n_size-1) {
		
		(norm+1..n_size).into_par_iter().for_each(|row| {
			let multiplier = a[row][norm]/a[norm][norm];
			
			(norm..n_size).into_par_iter().for_each(|col| {
				let mut at = a.clone();
				at[row][col] -= at[norm][col] * multiplier;
			});
			
			let mut bt = b.clone();
			bt[row] -= bt[norm] * multiplier;
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
