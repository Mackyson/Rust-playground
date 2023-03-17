use binary_search_tree::BST;

fn main() {
    let mut tree = BST::<i32>::new();

    tree.insert(1);
    tree.insert(2);
    tree.insert(0);
    tree.insert(3);
    tree.insert(4);
    tree.insert(-1);
    tree.insert(-2);
    assert!(tree.find(1));
    println!("{:#?}", tree);
    tree.delete(1);
    assert!(tree.find(-2));
    assert!(tree.find(-1));
    assert!(tree.find(0));
    assert!(!tree.find(1));
    assert!(tree.find(2));
    assert!(tree.find(3));
    assert!(tree.find(4));

    println!("{:#?}", tree);
}
