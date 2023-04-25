use std::thread;

pub fn gauss(n_size: usize, b: &mut Vec<f64>, x: &mut Vec<f64>, a: &mut Vec<Vec<f64>>, num_thread: usize){
	let mut threads_num = Vec::new();
	
	for norm in 0..(n_size-1) {
		for i in 0..(num_thread){
			let mut at = a.clone();
			let mut bt = b.clone();
			let t = thread::spawn(move ||{
				for row in norm+1..n_size {
					let multiplier = at[row][norm]/at[norm][norm];
					for col in norm..n_size {
						at[row][col] -= at[norm][col] * multiplier;
					}
					bt[row] -= bt[norm] * multiplier;
				}
				//print thread numbers t see if its working
				//println!("thread = {}", norm);
			});
			//print number of chunks of threads
			println!("chunk = {}", i);
			threads_num.push(t);
		}
	
	}
	
	for t in threads_num {
		t.join().expect("thread failed!");
	}
	
	
	for row in  (0..(n_size)).rev() {
		x[row] = b[row];
		for col in (row..(n_size)).rev() {
			x[row] -= a[row][col] * x[col];
		}
		x[row] /= a[row][row];
	}
}
