use crate::file_io::read;
use crate::sturcture::Node;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn encode(filename: String) -> Option<HashMap<String, String>> {
    println!("Encoding expanded code");
    let content: Option<String> = read(filename);
    match content {
        Some(content) => {
            let freq = count_freq(&content);
            let mut min_heap = BinaryHeap::new();
            for (key, value) in freq {
                let node = Node::leaf(key.to_string(), value);
                min_heap.push(Reverse(node));
            }
            ////println!(" min heap: {:?}", min_heap);
            while min_heap.len() > 1 {
                let min1: Node = min_heap.pop().unwrap().0;
                let min2 = min_heap.pop().unwrap().0;
                let merged = Node::merge(min1, min2);
                min_heap.push(Reverse(merged));
            }
            let huffman_tree = min_heap.pop().unwrap().0;
            ////println!("Huffman Tree: {:?}", huffman_tree);
            let mut res: HashMap<String, String> = HashMap::new();
            get_encoding(&huffman_tree, String::new(), &mut res);
            ////println!("\n\n\n\n\n Huffman Encoding Table: {:?}", res);
            return Some(res);
        }
        None => {
            eprintln!("No content to encode (failed to read ./to_encode/test.txt).");
            return None;
        }
    }
}

pub fn count_freq(content: &String) -> HashMap<char, u128> {
    let mut count: HashMap<char, u128> = HashMap::new();
    for character in content.chars() {
        *count.entry(character).or_insert(0) += 1;
    }
    //println!("{:?}", count);
    count
}

fn get_encoding(node: &Node, code: String, table: &mut HashMap<String, String>) {
    if node.is_leaf() {
        table.insert(node.token(), code.clone());
    }
    let left = node.left();
    let right = node.right();
    if left.is_some() {
        let mut left_code = code.clone();
        left_code.push('0');
        get_encoding(left.unwrap(), left_code, table);
    }
    if right.is_some() {
        let mut right_code = code.clone();
        right_code.push('1');
        get_encoding(right.unwrap(), right_code, table);
    }
}
