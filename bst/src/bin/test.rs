use bst::Bst;

fn main() {
    let mut bst = Bst::new();
    bst.insert(0);
    bst.insert(-1);
    bst.insert(1);
    println!("{:#?}", bst);
}
