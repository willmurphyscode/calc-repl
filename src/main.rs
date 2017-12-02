mod parse_node;

fn main() {
    let a = parse_node::ParseNode::Operand(32);
    println!("{}", a);
    println!("Hello, world!");
}
