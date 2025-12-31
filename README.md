# 🗜️ Parallel Huffman Compressor (Rust)

A **multi-threaded file compressor and decompressor** written in Rust.
It implements **Huffman encoding/decoding** and uses Rayon to process **multiple files in parallel**.

---
## Working

This project has two modes:

### 1) Encode (compress)
1. `main.rs` asks you to choose **encode** or **decode**.
2. In **encode** mode, it scans `./to_encode/` and processes every file in that folder in parallel.
3. For each file, `encoder.rs`:
   - counts character frequencies,
   - builds a Huffman tree (min-heap),
   - generates an encoding table (symbol -> bitstring).
4. `compress.rs` writes the compressed output into `./encoded_output/`:
   - `{original_filename}_encoded.bin`

Binary format (what gets stored inside `*_encoded.bin`):
- number of table entries (`u32`)
- for each entry:
  - key length (`u8`) + key bytes
  - code length (`u8`) + code bytes
- original symbol count (`u64`)
- encoded bitstream (packed into bytes)

There’s also a debug file (optional / for inspection):
- `{original_filename}_debug.bin`

### 2) Decode (decompress)
1. In **decode** mode, it scans `./to_decode/` and processes every file in that folder in parallel.
2. `decoder.rs` reads the binary format and reconstructs the Huffman table (bitstring -> symbol).
3. `expand.rs` reads:
   - the original symbol count
   - the packed bitstream
   then walks the bits until the symbol count is reached.
4. Output is written to `./decoded_output/`:
   - `{input_filename}_decoded.txt`

---
## File Structure

### Root
- `Cargo.toml` / `Cargo.lock`: Rust package metadata + dependencies
- `config.toml`: folder config (currently not wired into the main encode/decode flow)
- `README.md`: project description (this file)
- `LICENSE`: license info

### Input / Output folders
- `to_encode/`
  - put the text files you want to compress here
- `encoded_output/`
  - output: `*_encoded.bin` (compressed data)
  - output: `*_debug.bin` (debug info)
- `to_decode/`
  - put the `*_encoded.bin` files you want to decompress here
- `decoded_output/`
  - output: `*_decoded.txt`

### Source code (`src/`)
- `main.rs`: CLI entry point (encode/decode + parallel file processing)
- `path_read.rs`: lists all files in a folder
- `thread_pool.rs`: Rayon thread pool + parallel processing helper
- `file_io.rs`: reads input files (text/binary)
- `encoder.rs`: builds frequency map, Huffman tree, and encoding table
- `compress.rs`: writes Huffman table + encoded content into the binary format
- `decoder.rs`: reads the binary format and reconstructs the decoding table
- `expand.rs`: expands the encoded bitstream back into the original text
- `sturcture.rs`: Huffman tree `Node` + heap ordering (name is a typo but works)

---
## Usage

### 0) Build (download deps)
Rust/Cargo will download everything automatically the first time you build.

For a quick build:
```sh
cargo build
```

### 1) Quick run (trial)
Run directly with Cargo:
```sh
cargo run
```

It’ll ask you to type `encode` or `decode`.

### 2) Encode (compress)
1. Put files in: `to_encode/`
2. Run the program (`cargo run` or the release binary).
3. When prompted, type `encode`.
4. Outputs go to: `encoded_output/`

### 3) Decode (decompress)
1. Copy/move the `*_encoded.bin` files from `encoded_output/` into: `to_decode/`
2. Run the program (`cargo run` or the release binary).
3. When prompted, type `decode`.
4. Outputs go to: `decoded_output/`

### 4) Release build (final)
If you want the fast/optimized build:
```sh
cargo build --release
```

Then run:
```sh
./target/release/huffman-encoding
```

---
## Stats

### Encoding
- Input text size: ~800 mb (100 mb * 8 files)
- Encoded binary size: ~600 mb
- Compression ratio: ~25%
- Encoding time: ~7.8s (almost no load + 8 threads for 8 files)

### Decoding
- Using the previous compressed data for decompression
- Time taken: ~22.3s (same favourable conditions as above)

> Small note: this project can feel **fast** mainly because it’s a **simple Huffman implementation** and it currently focuses on **text files**.
> But the compression ratio is also **not as good** as mature tools like gzip/zstd — that’s the tradeoff.

---

This project was built mainly to learn Rust, so it’s not meant to compete with tools like gzip or zlib.
It hasn’t been optimized for speed or compression ratio — it’s just a Huffman coding implementation in Rust.

There’s still a lot of optimization potential (both algorithmic and code-level). One of the main bottlenecks right now is file I/O:
- the encoder currently reads the whole file into memory,
- and the compressed writer ends up reading the input again when producing the final binary output.

Feel free to explore, modify, and experiment with the code!
