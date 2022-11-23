#[derive(Debug)]
pub struct Node {
    pub symbol: Option<usize>,
    pub probability: f32,
    pub bit: Option<u8>,

    pub right: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
}

impl Node {
    pub fn new(
        symbol: Option<usize>,
        probability: f32,
        bit: Option<u8>,
        right: Option<Box<Node>>,
        left: Option<Box<Node>>,
    ) -> Node {
        Node {
            symbol,
            probability,
            bit,
            right,
            left,
        }
    }

    fn is_leaf(&self) -> bool {
        // Generically is leaf if both sides don't have children, in this application a Node has
        // either 0 or 2 children, so OR can be used instead of AND here.
        self.right.is_none() && self.left.is_none()
    }
    
    const LEFT_PATH_VALUE:  u8 = 1;
    const RIGHT_PATH_VALUE: u8 = 0;
    pub fn join_nodes(mut left_node: Box<Node>, mut right_node: Box<Node>) -> Box<Node> {
        let new_probability = right_node.probability + left_node.probability;
        right_node.bit = Some(Node::RIGHT_PATH_VALUE);
        left_node.bit = Some(Node::LEFT_PATH_VALUE);
        
        println!("joining p {} - {:?} and p {} - {:?}", right_node.probability, right_node.bit, left_node.probability, left_node.bit);

        Box::new(Node::new(None, new_probability, None, Some(right_node), Some(left_node)))
    }

    pub fn dfs_tree(my_node: Node, coding: &mut Vec< Option<Vec<u8>>>, aux: &mut Vec<u8>) { 
        aux.push(my_node.bit.unwrap());
        println!("Vector inside dfs {:?} - myprob: {}", aux, my_node.probability);
        if my_node.is_leaf() {
            coding[my_node.symbol.unwrap()] = Some(aux.clone());
            return;
        }

        Self::dfs_tree(*my_node.right.unwrap(), coding, aux);
        aux.pop();
        Self::dfs_tree(*my_node.left.unwrap(), coding, aux);
        aux.pop();
    }
}
