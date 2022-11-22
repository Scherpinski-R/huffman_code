use std::env;
use std::process;
use std::vec::Vec;

use crate::binary_tree::Node;

pub mod binary_tree;

fn main() {
    let args: Vec<String> = env::args().collect();
    let size = args.len();
    
    if size <= 1 {
        println!("Please gimme some numbers or else I'll have no job");
        process::exit(1);
    }

    let mut symbols: Vec<Box<Node>> = Vec::new();
    
    for i in 1..size {
        let number: f32 = args[i].parse().unwrap();
        println!("{}", number); 
        //TODO: Insert symbols probabilities in Vector
        //let val1 = Box::new(Node::new(0.42, None, None, None));
        //let val2 = Box::new(Node::new(0.16, None, None, None));
    } 
    
    //let new_value = Node::join_nodes(val1, val2);

    loop {
        let n = symbols.len();
        
        if n < 2 {
            break;
        }
        // TODO: Remove the last 2 elements ( smaller probability ) 
        // join_nodes -> insert new in Vector
    }
}
