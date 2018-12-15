#![allow(warnings)]

mod tree;

use tree::Forest;

fn main() {
    let mut f = Forest::new();
    let root = f.get_root();
    let child = root.append("child", &mut f);
    child.append("child of child", &mut f);

    child.append("child of child2", &mut f);

    let child2 = root.append("child2", &mut f);
    child2.append("child2 of child2", &mut f);

    println!("{}", f);
}
