# 🗜️ Parallel Huffman Compressor (Rust)

A **multi-threaded file compressor and decompressor** written in Rust, implementing **Huffman encoding/decoding** and a **custom thread pool** to process multiple files in parallel. 

( Trying to do the treadpool currently single file - single thread :) 

---
## Working

This project has two modes:

### 1) Encode (compress)
1. `main.rs` loads `config.toml` and asks you to choose **encode** or **decode**.
2. In **encode** mode, it calls `compress::write_compressed(&config)`.
3. `file_io::read(&config)` reads the input text file from:
   - `encode_input_dir/input.txt`
4. `encoder::encode(&config)`:
   - counts character frequencies,
   - builds a Huffman tree using a min-heap,
   - generates a Huffman encoding table (character -> bitstring).
5. `compress::write_compressed(&config)` writes a binary output file to:
   - `encoded_output_dir/output.bin`

The binary format is:
- number of table entries (u32)
- for each entry: key length (u8), key bytes, code length (u8), code bytes
- original symbol count (u64)
- encoded bitstream packed into bytes

It also writes a debug file to:
- `encoded_output_dir/output_debug.bin` (human-readable info like table + bitstream)

### 2) Decode (decompress)
1. In **decode** mode, `main.rs` calls `expand::write_expanded(&config)`.
2. `decoder::decode(&config)` reads the encoded binary from:
   - `decode_input_dir/input.bin`
3. `decoder.rs` reconstructs the Huffman table (bitstring -> character).
4. `expand.rs` reads the original symbol count + bitstream, then walks bits until the symbol count is reached.
5. The final decoded text is written to:
   - `decoded_output_dir/output.txt`

---
## File Structure

### Root
- `Cargo.toml` / `Cargo.lock`: Rust package metadata + dependencies
- `config.toml`: where input/output folders are configured
- `README.md`: project description (this file)
- `LICENSE`: license info

### Input / Output folders
- `to_encode/`
  - `input.txt` (text file you want to compress)
- `encoded_output/`
  - `output.bin` (compressed binary output)
  - `output_debug.bin` (debug info about the encoding)
- `to_decode/`
  - `input.bin` (binary file you want to decompress; often copied from `encoded_output/output.bin`)
- `decoded_output/`
  - `output.txt` (final decompressed text)

### Source code (`src/`)
- `main.rs`: CLI entry point; loads config and dispatches to encode/decode
- `config.rs`: reads `config.toml` into a `Config` struct
- `file_io.rs`: reads input files (text/binary) using config directory paths
- `encoder.rs`: builds frequency map, Huffman tree, and encoding table
- `compress.rs`: writes encoding table + encoded content into the compressed binary format
- `decoder.rs`: reads the binary format and reconstructs the decoding table
- `expand.rs`: expands the encoded bitstream back into the original text
- `sturcture.rs`: defines the Huffman tree `Node` and heap ordering

---
## Usage

### 1) Configure paths
Edit `config.toml` to point to your folders:
- `encode_input_dir` (contains `input.txt`)
- `encoded_output_dir` (where `output.bin` and `output_debug.bin` will be written)
- `decode_input_dir` (contains `input.bin`)
- `decoded_output_dir` (where `output.txt` will be written)

Note: the code currently expects these fixed filenames:
- encode input: `input.txt`
- decode input: `input.bin`

### 2) Encode (compress)
1. Put your text file at: `to_encode/input.txt` (or whatever `encode_input_dir` is set to).
2. Run the program.
3. When prompted, type `encode`.
4. Outputs:
   - `encoded_output/output.bin` (compressed data)
   - `encoded_output/output_debug.bin` (debug info)

### 3) Decode (decompress)
1. Copy/move `encoded_output/output.bin` to `to_decode/input.bin` (or whatever `decode_input_dir` is set to).
2. Run the program.
3. When prompted, type `decode`.
4. Output:
   - `decoded_output/output.txt` (decompressed text)

---
## Stats

### Encoding 
- Input text size: ~100 mb
- Encoded binary size: ~75 mb
- Compression ratio: ~25%
- encoding time : ~103.890139684s (pc on medium load + lot of debugging info written + extra debugging file )

---

This project was built mainly to learn Rust, so it’s not meant to compete with tools like gzip or zlib.
It hasn’t been optimized for speed or compression ratio—it's just a Huffman coding implementation in Rust.

Feel free to explore, modify, and experiment with the code!
