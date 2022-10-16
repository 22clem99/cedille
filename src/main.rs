mod ccedille;

extern crate hime_redist;

use std::env;

use hime_redist::ast::AstNode;
use std::io;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let result = ccedille::parse_string("\"azerty\"");
    let ast = result.get_ast();
    let root = ast.get_root();
    print(root, Vec::<bool>::new());
}

fn print<'a>(node: AstNode<'a>, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{:}", node);
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), child_crossings);
        i += 1;
    }
}