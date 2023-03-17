// 写経: https://docs.rs/binary_search_tree/latest/src/binary_search_tree/lib.rs.html#494-500
use std::fmt::Debug;

#[derive(Debug)]
struct BSTNode<T: Ord> {
    data: T,
    left: BST<T>,
    right: BST<T>,
}

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<Box<BSTNode<T>>>,
}

impl<T: Ord> BSTNode<T> {
    fn new(data: T) -> BSTNode<T> {
        BSTNode {
            data: data,
            left: BST::new(),
            right: BST::new(),
        }
    }
}
impl<T: Ord> BST<T> {
    pub fn new() -> BST<T> {
        BST::<T> { root: None }
    }
    pub fn insert(&mut self, data: T) -> bool {
        let mut target = self;
        loop {
            match target.root {
                None => break,
                Some(ref mut node) => match node.data.cmp(&data) {
                    std::cmp::Ordering::Less => target = &mut node.right,
                    std::cmp::Ordering::Equal => return false,
                    std::cmp::Ordering::Greater => target = &mut node.left,
                },
            }
        }
        target.root = Some(Box::new(BSTNode::new(data)));
        true
    }
    pub fn find(&self, data: T) -> bool {
        let mut target = self;
        loop {
            match target.root {
                None => return false,
                Some(ref node) => match node.data.cmp(&data) {
                    std::cmp::Ordering::Less => target = &node.right,
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => target = &node.left,
                },
            }
        }
    }
    pub fn delete(&mut self, data: T) -> bool {
        //let mut target = self;
        let mut target: *mut BST<T> = self;
        unsafe {
            loop {
                match (*target).root {
                    None => return false,
                    Some(ref mut node) => match node.data.cmp(&data) {
                        std::cmp::Ordering::Less => target = &mut node.right,
                        std::cmp::Ordering::Greater => target = &mut node.left,
                        std::cmp::Ordering::Equal => {
                            match (&node.left.root, &node.right.root) {
                                //左右に子無し，
                                (None, None) => {
                                    (*target).root = None;
                                }
                                //左の子のみあり
                                (Some(_), None) => {
                                    (*target).root = node.left.root.take();
                                    //(*target).root = left;
                                }
                                //右の子のみあり
                                (None, Some(_)) => {
                                    (*target).root = node.right.root.take();
                                }
                                //左右に子あり
                                (Some(_), Some(_)) => {
                                    match node.right.extract_min() {
                                        None => panic!("deletion error"), //return false, //なぜか右の部分木が空
                                        Some(min_data) =>
                                        //target.root.as_mut().unwrap().data = min_val,
                                        {
                                            (*target).root.as_mut().unwrap().data = min_data
                                        }
                                    }
                                }
                            };
                            return true;
                        }
                    },
                }
            }
        }
    }

    fn extract_min(&mut self) -> Option<T> {
        let mut min = None;
        let mut target = self;
        if let None = target.root {
            return None;
        };
        while let Some(_) = target.root.as_mut().unwrap().left.root {
            target = &mut target.root.as_mut().unwrap().left;
        }
        let node = target.root.take().unwrap();
        min = Some(node.data);
        target.root = node.right.root;

        return min;
    }
}
