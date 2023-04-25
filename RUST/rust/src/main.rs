//RUSTsequential.rs
mod sequential;
mod threads;
mod chunk;
mod parallel;
mod pool;
mod chunk_pool;

use std::env;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use std::time::Instant;

fn main(){
	
	let args: Vec<String> = env::args().collect();
	if args.len() < 5 || args.len() > 5 {
		println!("Not enough arguments.");
	}
	
	let n = &args[1];
	let seed = &args[2];
	let condition = &args[3];
	let n_t = &args[4];
	
	let n_size: usize = n.parse().unwrap();
	let s_size: u64 = seed.parse().unwrap();
	let c: usize = condition.parse().unwrap();
	let num_thread: usize = n_t.parse().unwrap();
	
	let mut b = vec![0.0;n_size];
	let mut x = vec![0.0;n_size];
	
	let mut a = vec![vec![0.0;n_size];n_size];
	
	println!("Matriz dimension size: { }. \nSeed: { }", n_size, s_size);
	
	//Initializing A, B and X
	let mut r = StdRng::seed_from_u64(s_size);
	
	for i in 0..n_size {
		for j in 0..n_size {
			let v: f64 = r.gen();
			a[j][i] = v/32768.0;
		}
		let vv: f64 = r.gen();
		b[i] = vv/32768.0;
		x[i] = 0.0;
	}
	
	// Print Inputs
	printin(n_size, &mut b, &mut x, &mut a);
	
	if c == 1{ //sequential
		let start = Instant::now();
		// Gauss Elimination
		sequential::gauss(n_size, &mut b, &mut x, &mut a);
		let end = start.elapsed();
		println!("Time elapsed: {:?}", end);
		println!("Threads Number: 0");
	}else if c == 2{ //threads
		let start = Instant::now();
		// Gauss Elimination
		threads::gauss(n_size, &mut b, &mut x, &mut a);
		let end = start.elapsed();
		println!("Time elapsed: {:?}", end);
		println!("Threads Number: 0");
	} else if c == 3{ //chunk of threads
		let start = Instant::now();
		// Gauss Elimination
		chunk::gauss(n_size, &mut b, &mut x, &mut a, num_thread);
		let end = start.elapsed();
		println!("Time elapsed: {:?}", end);
		println!("Threads Number: {}", num_thread);
	} else if c == 4{ // parallel -- using Rayon
		let start = Instant::now();
		// Gauss Elimination
		parallel::gauss(n_size, &mut b, &mut x, &mut a, num_thread);
		let end = start.elapsed();
		println!("Time elapsed: {:?}", end);
		println!("Threads Number: 0");
	} else if c == 5{ // pool of threads
		let start = Instant::now();
		// Gauss Elimination
		pool::gauss(n_size, &mut b, &mut x, &mut a, num_thread);
		let end = start.elapsed();
		println!("Time elapsed: {:?}", end);
		println!("Threads Number: {}", num_thread);
	} else if c == 6{ // pool of threads in chunks
		let start = Instant::now();
		// Gauss Elimination
		chunk_pool::gauss(n_size, &mut b, &mut x, &mut a, num_thread);
		let end = start.elapsed();
		println!("Time elapsed: {:?}", end);
		println!("Threads Number: {}", num_thread);
	} else if c > 6 || c < 1{
		println!("Usage: 1 - sequential, 2 - threads, 3 - chunks, 4 - parallel, 5 - pool of threads, 6 - pool with chunk");	
	}
	//Print outout X
	printout(n_size, &mut x);
}

fn printin(n_size: usize, b: &mut Vec<f64>, x: &mut Vec<f64>, a: &mut Vec<Vec<f64>>){
	println!("--A--");
	for i in 0..n_size {
		for j in 0..n_size {
			print!("[{}]", a[i][j]);
		}
		print!("\n");
	}
	println!("--B--");
	for i in 0..n_size {
		println!("[{}]", b[i]);
	}
	println!("--X--");
	for i in 0..n_size {
		println!("[{}]", x[i]);
	}
}

fn printout(n_size: usize, x: &mut Vec<f64>){
	println!("--X--");
	for i in 0..n_size {
		println!("[{}]", x[i]);
	}

}

