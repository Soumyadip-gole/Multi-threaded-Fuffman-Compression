# 🗜️ Parallel Huffman Compressor (Rust)

A **multi-threaded file compressor and decompressor** written in Rust, implementing **Huffman encoding/decoding** and a **custom thread pool** to process multiple files in parallel. ( Trying to do the treadpool currently single file - single thread :) 
---

## ✨ Features

* ✅ **Huffman Encoding & Decoding**

  * Byte-frequency analysis
  * Huffman tree construction
  * Bit-level encoding and decoding
  * Lossless round-trip verification

* 🧵 **Custom Thread Pool**  (to be done)

  * Fixed-size worker pool
  * Threads are spawned once and reused
  * Workers wait for tasks instead of exiting
  * Controlled concurrency (no thread explosion)

* ⚡ **Parallel File Processing**  (to be done)

  * One file = one task
  * Multiple files compressed / decompressed concurrently
  * Scales with available CPU cores

* 📊 **Performance-Oriented**   (to be done)

  * CPU-bound workload
  * Benchmarkable against single-threaded execution

---


### Parallelism Model (expected)

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

## 🗂️ Project Structure (undecided)

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

## 🚀 Usage (undecided)

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

---

## 🐞 Debugging: `output.bin` binary layout

When you run compression (`write_compressed()` in `src/compress.rs`), the program writes a binary file to `./to_decode/output.bin`.

This section documents the exact byte layout so you can inspect the file with a hex viewer or write your own decoder.

### Endianness

* All multi-byte integers are **big-endian** (network byte order) because the code uses `to_be_bytes()`.

### File format (in order)

1) **Encoding table length**

* **32 bits (4 bytes)**: `N` = number of Huffman table entries (`u32`, big-endian)

2) **Encoding table entries** (repeated `N` times)

For each entry:

* **8 bits (1 byte)**: `K` = key length in bytes (`u8`)
* **K bytes**: key bytes (currently written as UTF-8 bytes from `String::as_bytes()`)
* **8 bits (1 byte)**: `V` = code length in bytes (`u8`)
* **V bytes**: code bytes (ASCII `'0'`/`'1'` characters; this is a *debug-friendly* representation)

Notes:

* In the current implementation the table key is a `String` created via `ch.to_string()`. For normal text this is typically a single UTF-8 character, but the format supports longer UTF-8 sequences because we store `K` + raw bytes.
* The code length is stored in **bytes**, not bits. Since the code is written as a string of `'0'`/`'1'`, bytes == bits count.

3) **Original symbol count**

* **64 bits (8 bytes)**: `original_len` = number of Unicode scalar values (`input.chars().count() as u64`, big-endian)

This is used by decoding to know when to stop emitting symbols (because the final byte of the bitstream may contain padding).

4) **Encoded bitstream**

* Remaining bytes until EOF are the compressed data bits, packed into bytes.

Packing rule used by the current encoder:

* Bits are processed left-to-right.
* For each bit, the buffer is shifted left by 1 and the bit is OR’d in.
* Every 8 bits, one byte is written.
* If there are leftover bits at the end, the last byte is left-shifted to fill to 8 bits and written once.

In other words:

* The **first encoded bit becomes the most-significant bit** of the first byte.
* The final byte may include **zero padding** in the least significant bits.

### Debug sidecar file

For human-readable inspection, compression also writes `./to_decode/output_debug.bin` (despite the `.bin` extension, it’s plain text) containing:

* the Huffman table
* the original symbol count
* the full encoded bitstream as a `0/1` string

### Worked example

Assume the input file contains:

* `test`

And the generated Huffman table (example) is:

* `t` → `0`
* `s` → `10`
* `e` → `11`

The beginning of `output.bin` may look like this:

```bash
$ hexdump -C output.bin | head
00000000  00 00 00 03 01 73 02 31  30 01 65 02 31 31 01 74  |.....s.10.e.11.t|
00000010  01 30 00 00 00 00 00 00  00 04 ...
```

How to read that dump:

* `00 00 00 03` → `N = 3` table entries (`u32` big-endian)

Then each entry is `K (1 byte) + key (K bytes) + V (1 byte) + value (V bytes)`:

* `01 73 02 31 30`
  * `01` → `K = 1`
  * `73` → key = `"s"` (ASCII `0x73`)
  * `02` → `V = 2`
  * `31 30` → value = `"10"` (ASCII `0x31 0x30`)

* `01 65 02 31 31`
  * `01` → `K = 1`
  * `65` → key = `"e"`
  * `02` → `V = 2`
  * `31 31` → value = `"11"`

* `01 74 01 30`
  * `01` → `K = 1`
  * `74` → key = `"t"`
  * `01` → `V = 1`
  * `30` → value = `"0"`

Immediately after the table comes the original symbol count:

* next **8 bytes** → `original_len` (`u64` big-endian). For input `test`, this value is `4`.

After that, the remainder of the file is the packed encoded bitstream.

> Note: the **order of table entries in the hexdump is not guaranteed**, because Rust’s `HashMap` iteration order is randomized. The example above is just one possible ordering.
