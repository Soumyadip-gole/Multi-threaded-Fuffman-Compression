# 🗜️ Parallel Huffman Compressor (Rust)

A **multi-threaded file compressor and decompressor** written in Rust, implementing **Huffman encoding/decoding** and a **custom thread pool** to process multiple files in parallel.

This project focuses on **systems fundamentals**: concurrency, task scheduling, and CPU-bound parallelism — not on inventing a new compression algorithm.

---

## ✨ Features

* ✅ **Huffman Encoding & Decoding**

  * Byte-frequency analysis
  * Huffman tree construction
  * Bit-level encoding and decoding
  * Lossless round-trip verification

* 🧵 **Custom Thread Pool**

  * Fixed-size worker pool
  * Threads are spawned once and reused
  * Workers wait for tasks instead of exiting
  * Controlled concurrency (no thread explosion)

* ⚡ **Parallel File Processing**

  * One file = one task
  * Multiple files compressed / decompressed concurrently
  * Scales with available CPU cores

* 📊 **Performance-Oriented**

  * CPU-bound workload
  * Benchmarkable against single-threaded execution

---

## 🧠 Design Overview

### Why Huffman Coding?

Huffman coding is a classic entropy encoding algorithm that:

* Is simple enough to implement correctly
* Clearly demonstrates compression fundamentals
* Is widely used as a building block in real compressors

This project intentionally avoids more complex algorithms (LZMA, arithmetic coding) to keep the focus on **concurrency and correctness**.

---

### Why a Thread Pool?

Creating threads repeatedly (`thread::spawn`) is expensive and unscalable.

Instead, this project uses:

* A **fixed number of worker threads**
* A **shared task queue**
* Workers that continuously:

  1. wait for a task
  2. execute it
  3. return to waiting

This mirrors how real-world CPU-bound systems handle parallel work.

---

### Parallelism Model

```
Main Thread
 ├── submit file1
 ├── submit file2
 ├── submit file3

Thread Pool (N workers)
 ├── Worker 1 → Huffman(file1)
 ├── Worker 2 → Huffman(file2)
 ├── Worker 3 → Huffman(file3)
 └── Worker 4 → waiting
```

* No shared mutable state between tasks
* No locking during compression work
* Safe and predictable parallelism

---

## 🗂️ Project Structure

```
src/
 ├── huffman/
 │   ├── frequency.rs    # Frequency table
 │   ├── tree.rs         # Huffman tree
 │   ├── encode.rs       # Encoding logic
 │   └── decode.rs       # Decoding logic
 │
 ├── thread_pool/
 │   ├── mod.rs
 │   └── pool.rs         # Custom thread pool
 │
 ├── cli.rs              # Argument parsing
 └── main.rs             # Entry point
```

---

## 🚀 Usage

### Compress files

```bash
cargo run -- compress input_dir/ output_dir/
```

### Decompress files

```bash
cargo run -- decompress input_dir/ output_dir/
```

Each file is processed independently and may be handled by a different worker thread.

---

## 📈 Performance Notes

* Parallel speedup depends on:

  * number of CPU cores
  * number of input files
  * file sizes

* Huffman tree construction is sequential **per file**, but **files are processed in parallel**.

This mirrors how many real-world compressors parallelize work at the **job level**, not inside the core algorithm.

---

## ⚠️ Limitations

* Huffman-only compression (no dictionary-based compression)
* Not designed to beat tools like 7-Zip or WinRAR
* Optimized for learning and correctness, not maximum compression ratio

---

## 🎯 Learning Outcomes

This project demonstrates:

* Safe concurrency in Rust
* Thread pool design
* CPU-bound parallel task execution
* Bit-level data processing
* Clear separation of concerns

---

## 🧩 Future Improvements (Optional)

* Block-level compression for large files
* Progress reporting using atomics
* Benchmark harness
* Panic recovery in worker threads

---

## 📌 Final Note

> This project is not about inventing new compression algorithms —
> it is about **understanding and implementing real systems concepts correctly**.
