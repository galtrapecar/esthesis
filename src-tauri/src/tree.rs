use rand::Rng;

enum NodeType {
    Function,
    Terminal,
}

struct Node {
    node_type: NodeType,
    function: Option<String>,
    terminal: Option<String>,
    value: String,
    args: Vec<Node>
}

fn grow(depth: u32, max_depth: u32) {
    // initial node


    // if d == D return random terminal
    let f_or_t = rand::thread_rng().gen_range(0..=10);

}