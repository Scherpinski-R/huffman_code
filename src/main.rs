use std::env;
use std::process;
use std::vec::Vec;

use crate::binary_tree::Node;

pub mod binary_tree;

fn find_position(v: &Vec<Box<Node>>, k: f32) -> usize {
    let mut pos: usize = 0; 
    while pos < v.len() {
        if k < v[pos].probability {
            return pos; 
        }

        pos = pos + 1;
    }

    return pos;
}

const MIN_PROBABILITY_SUM: f32 = 0.99;
fn ensure_probability_sum(args: &Vec<String>, size: usize) {
    let mut acc: f32= 0.0;
    for i in 1..size {
        let number: f32 = args[i].parse().unwrap();
        acc = acc + number;
    }

    if acc < MIN_PROBABILITY_SUM {
        println!("Please ensure the sum of probabilities is at least {}", MIN_PROBABILITY_SUM);
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size = args.len();
    
    if size <= 1 {
        println!("Please gimme some numbers or else I'll have no job");
        process::exit(1);
    }

    // Quit if probability given is < MIN_PROBABILITY_SUM 
    ensure_probability_sum(&args, size);

    let mut symbols: Vec<Box<Node>> = Vec::new();
    
    for i in 1..size {
        let number: f32 = args[i].parse().unwrap();
        let pos = find_position(&symbols, number); 
        symbols.insert(pos, Box::new(Node::new(Some(i-1), number, None, None, None)));
    } 
   
    loop {
        let n = symbols.len();
        
        if n < 2 {
            break;
        }
        
        let f_node = symbols.remove(n-1);
        let s_node = symbols.remove(n-2);

        let new_value = Node::join_nodes(f_node, s_node);
        let pos = find_position(&symbols, new_value.probability);
        symbols.insert(pos, new_value);
    }
    
    //TODO: Run throught the Tree and display code for each symbol(i)
}
