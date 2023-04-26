# Gauss Elimination - RUST language


## Compile and Run

The compilation and execution must be made inside rust directory!

### LINUX - Windows
Compile:

>	cargo build

Run:

>	cargo run [matrix size] [Seed] [choice] [number of threads]

Number of threads is only needed for: Chunk, ThreadPool and ThreadPool with chunk. The number of threads can't be 0 for them.

### Choice:

1.Serial
> sequential.rs
2.Threads
> threads.rs
3.Chunk
> chunk.rs
4.Parallel Rayon
> parallel.rs
5.ThreadPool using Rayon
> pool.rs
6.ThreadPool with chunk using Rayon
> chunk_pool.rs
