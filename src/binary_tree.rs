pub struct Node {
    pub probability: f32,
    pub bit: Option<u8>,

    pub right: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
}

impl Node {
    pub fn new(
        probability: f32,
        bit: Option<u8>,
        right: Option<Box<Node>>,
        left: Option<Box<Node>>,
    ) -> Node {
        Node {
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

    pub fn join_nodes(mut f_node: Box<Node>, mut s_node: Box<Node>) -> Box<Node> {
        let new_probability = f_node.probability + s_node.probability;
        // TODO: Create macro for default RightValue, LeftValue
        f_node.bit = Some(1);
        s_node.bit = Some(0);

        Box::new(Node::new(new_probability, None, Some(f_node), Some(s_node)))
    }
}