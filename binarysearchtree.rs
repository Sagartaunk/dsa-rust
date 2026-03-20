use std::cmp::Ordering;

struct Node<T:Ord + Clone> { //Left can strictly be smaller and right can strictly be greater when compared to value
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}
struct Tree<T:Ord + Clone> {
    top: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> Tree<T>{
    fn new() -> Self{ //Arbitrary Value Pointer
        Self{
            top:None
        }
    }

    fn insert(&mut self , val : T){
        if self.top.is_none() {
            self.top = Some(Box::new(Node{value:val , left:None , right:None}));
            return;
        }

        let mut leaf = self.top.as_mut().unwrap();

        loop {
            match val.cmp(&leaf.value){
                Ordering::Less => {
                    if leaf.left.is_none(){
                        leaf.left = Some(Box::new(Node{value:val.clone(),left:None,right:None}));
                        return;
                    }
                    leaf = leaf.left.as_mut().expect("Error : 1");
                    continue;
                }
                Ordering::Greater => {
                    if leaf.right.is_none(){
                        leaf.right = Some(Box::new(Node{value:val.clone(),left:None,right:None}));
                        return;
                    }
                    leaf = leaf.right.as_mut().expect("Error : 2");
                    continue;
                }
                Ordering::Equal => {
                    break;
                }
            }
        }
    }

    fn contains(&self , val : &T) -> bool {
        if self.top.is_none() {
            return false;
        }

        let mut leaf = self.top.as_ref().unwrap();

        loop{
            match leaf.value.cmp(val){
                Ordering::Less => {
                    if leaf.right.is_none(){
                        return false;
                    }
                    leaf = leaf.right.as_ref().expect("Error : 3");
                    continue;
                }
                Ordering::Greater => {
                    if leaf.left.is_none(){
                        return false;
                    }
                    leaf=leaf.left.as_ref().expect("Error : 4");
                    continue;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
    }

    fn inorder(&self) -> Vec<T> {
        let mut storage : Vec<T> = Vec::new();
        if self.top.is_none(){
            return vec![];
        }
        branchprint(self.top.as_ref() , &mut storage);
        return storage;
    }
    fn hieght(&self) -> usize {
        high(self.top.as_ref())
    }
    fn delete(&mut self , key : T) {
        self.top = delete_node(self.top.take(), &key);
    }
    fn is_balanced(&self) -> bool {
        check_balance(self.top.as_ref())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(4);

        assert!(tree.contains(&5));
        assert!(tree.contains(&3));
        assert!(tree.contains(&7));
        assert!(tree.contains(&1));
        assert!(tree.contains(&4));
        assert!(!tree.contains(&0));
        assert!(!tree.contains(&6));
        assert!(!tree.contains(&100));
    }
}

#[test]
fn test_inorder() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    assert_eq!(tree.inorder(), vec![1, 3, 4, 5, 7]);
}

#[test]
fn test_height() {
    let mut tree = Tree::new();
    assert_eq!(tree.hieght(), 0);
    tree.insert(5);
    assert_eq!(tree.hieght(), 1);
    tree.insert(3);
    tree.insert(7);
    assert_eq!(tree.hieght(), 2);
    tree.insert(1);
    assert_eq!(tree.hieght(), 3);
}
#[test]
fn test_delete() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);

    // delete leaf
    tree.delete(1);
    assert!(!tree.contains(&1));
    assert_eq!(tree.inorder(), vec![3, 4, 5, 7]);

    // delete node with one child
    tree.delete(3);
    assert!(!tree.contains(&3));
    assert_eq!(tree.inorder(), vec![4, 5, 7]);

    // delete node with two children
    tree.delete(5);
    assert!(!tree.contains(&5));
    assert_eq!(tree.inorder(), vec![4, 7]);

    // delete root
    tree.delete(7);
    tree.delete(4);
    assert_eq!(tree.inorder(), vec![]);
}
#[test]
fn test_is_balanced() {
    let mut tree = Tree::new();
    assert!(tree.is_balanced());

    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    assert!(tree.is_balanced());

    tree.insert(1);
    tree.insert(4);
    assert!(tree.is_balanced());

    // skewed tree - insert in order
    let mut skewed = Tree::new();
    skewed.insert(1);
    skewed.insert(2);
    skewed.insert(3);
    skewed.insert(4);
    assert!(!skewed.is_balanced());
}


fn main(){

}
fn branchprint<T: Ord + Clone>(node :Option<&Box<Node<T>>> , storage:&mut Vec<T>){
    if let Some(n) = node {
        branchprint(n.left.as_ref() ,storage);
        storage.push(n.value.clone());
        branchprint(n.right.as_ref() ,storage);
    }
}
fn high<T: Ord + Clone>(node: Option<&Box<Node<T>>>) -> usize {
    match node {
        Some(n) => {
            let left = high(n.left.as_ref());
            let right = high(n.right.as_ref());
            1 + std::cmp::max(left, right)
        }
        None => 0,
    }
}
fn find_min<T:Ord + Clone>(node: &Option<Box<Node<T>>>) -> T {
    let mut current = node.as_ref().unwrap();

    while let Some(ref left) = current.left {
        current = left;
    }

    current.value.clone()
}
fn delete_node<T:Ord + Clone>(node: Option<Box<Node<T>>>, key: &T) -> Option<Box<Node<T>>> {
    match node {
        None => None,
        Some(mut n) => {
            match key.cmp(&n.value) {
                Ordering::Less => {
                    n.left = delete_node(n.left,key);
                    Some(n)
                }
                Ordering::Greater => {
                    n.right = delete_node(n.right, key);
                    Some(n)
                }
                Ordering::Equal => {
                    if n.left.is_none() {
                        return n.right;
                    }
                    if n.right.is_none() {
                        return n.left;
                    }
                    let min_val = find_min(&n.right);
                    n.value = min_val.clone();
                    n.right = delete_node(n.right,&min_val);
                    Some(n)
                }
            }
        }
    }
}
fn check_balance<T: Ord + Clone>(node: Option<&Box<Node<T>>>) -> bool {
    match node {
        None => true,
        Some(n) => {
            let left_height = high(n.left.as_ref());
            let right_height = high(n.right.as_ref());
            if (left_height as isize - right_height as isize).abs() > 1 {
                return false;
            }
            check_balance(n.left.as_ref()) && check_balance(n.right.as_ref())
        }
    }
}
