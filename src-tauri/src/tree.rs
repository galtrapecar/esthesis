
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