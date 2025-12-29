use crate::encoder::encode;
use std::fs::File;
use std::io::{Write, BufWriter};
use crate::file_io::read;

pub fn write_compressed() {
    let encoding_table = encode();

    match encoding_table {
        Some(encoding_table) => {
            let file = File::create("./to_decode/compressed.bin").unwrap();
            let file_test=File::create("./to_decode/output_debug.bin").unwrap();
            let mut writer = BufWriter::new(file);

            // ---------- DEBUGGING PURPOSES ONLY ----------
            let mut debug_writer = BufWriter::new(file_test);

            writeln!(debug_writer, "Huffman Debug Output").unwrap();
            writeln!(debug_writer, "-------------------").unwrap();

            // ---------- WRITE HUFFMAN TABLE ----------
            writeln!(
                debug_writer,
                "Number of entries: {}\n",
                encoding_table.len()
            ).unwrap();

            writeln!(debug_writer, "Huffman Table:").unwrap();

            for (key, value) in &encoding_table {
                writeln!(
                    debug_writer,
                    "'{}' ({} bytes) -> {} ({} bits)",
                    key,
                    key.len(),
                    value,
                    value.len()
                ).unwrap();
            }

            // ---------- WRITE ORIGINAL SYMBOL COUNT ----------
            let input = read().unwrap();
            let original_len = input.chars().count();

            writeln!(
                debug_writer,
                "\nOriginal symbol count: {}\n",
                original_len
            ).unwrap(); // Original Length in 8 bytes

            // ---------- WRITE ENCODED BITSTREAM ----------
            writeln!(debug_writer, "Encoded bitstream:").unwrap();

            let mut bitstream = String::new();

            for ch in input.chars() {
                let code = &encoding_table[&ch.to_string()];
                bitstream.push_str(code);
            }

            writeln!(debug_writer, "{}", bitstream).unwrap();

            debug_writer.flush().unwrap();


            //Actual Writing to binary file


            // ---------- WRITE HUFFMAN TABLE ----------
            let no_of_entries = encoding_table.len() as u32;
            writer.write_all(&no_of_entries.to_be_bytes()).unwrap();

            for (key, value) in &encoding_table {
                let key_bytes = key.as_bytes();
                writer.write_all(&[key_bytes.len() as u8]).unwrap();
                writer.write_all(key_bytes).unwrap();

                let value_bytes = value.as_bytes();
                writer.write_all(&[value_bytes.len() as u8]).unwrap();
                writer.write_all(value_bytes).unwrap();
            }

            // ---------- WRITE ORIGINAL SYMBOL COUNT ----------
            let input = read().unwrap();
            let original_len = input.chars().count() as u64;
            writer.write_all(&original_len.to_be_bytes()).unwrap();

            // ---------- WRITE ENCODED BITSTREAM ----------
            let mut buffer: u8 = 0;
            let mut count: u8 = 0;
            //println!("{}",input);
            //println!("Starting compression...");

            for ch in input.chars() {
                let code = &encoding_table[&ch.to_string()];

                for bit in code.chars() {
                    //println!("{}",bit);
                    buffer <<= 1;
                    if bit == '1' {
                        buffer |= 1;
                    }
                    count += 1;

                    if count == 8 {
                        writer.write_all(&[buffer]).unwrap();
                        buffer = 0;
                        count = 0;
                    }
                }
            }

            // ---------- FINAL FLUSH (ONLY ONCE) ----------
            if count > 0 {
                buffer <<= 8 - count;
                writer.write_all(&[buffer]).unwrap();
            }

            writer.flush().unwrap();
            //println!("Compression complete: output.bin written");
        }

        None => {
            //println!("No Encoding Table Generated");
        }
    }
}
