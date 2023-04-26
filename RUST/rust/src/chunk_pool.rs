use std::thread;

pub fn gauss(n_size: usize, b: &mut Vec<f64>, x: &mut Vec<f64>, a: &mut Vec<Vec<f64>>, num_thread: usize){
	let pool = rayon::ThreadPoolBuilder::new().num_threads(num_thread).build().unwrap();
	
	for norm in 0..(n_size-1) {
		let at = a.clone();
		let bt = b.clone();
		pool.install(move ||{
			let mut threads_num = Vec::new();
			for _ in 0..(num_thread){
				let mut att = at.clone();
				let mut btt = bt.clone();
				let t = thread::spawn(move ||{
					for row in norm+1..n_size {
						let multiplier = att[row][norm]/att[norm][norm];
						for col in norm..n_size {
							att[row][col] -= att[norm][col] * multiplier;
						}
						btt[row] -= btt[norm] * multiplier;
					}
					//print thread numbers t see if its working
					//println!("thread = {}", norm);
				});
				//print number of chunks of threads
				//println!("chunk = {}", i);
				threads_num.push(t);
			}
			for t in threads_num {
	 			t.join().expect("thread failed!");
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
