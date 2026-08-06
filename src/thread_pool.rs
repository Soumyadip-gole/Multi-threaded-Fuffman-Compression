use rayon::ThreadPoolBuilder;
use rayon::prelude::*;

// Initialize thread pool with specified number of threads
pub fn init_thread_pool(num_threads: usize) {
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
}

// Process files in parallel - pass your compression function
pub fn process_files_parallel<F>(files: Vec<String>, process_fn: F)
where
    F: Fn(String) + Sync + Send,
{
    files.par_iter().for_each(|file| {
        println!("Processing file: {}", file);
        process_fn(file.clone());
    });
}
