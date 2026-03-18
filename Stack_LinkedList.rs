
use std::fmt;

//Defination for structs
struct Node<T> {
    value : T,
    next : Option<Box<Node<T>>>,
}
struct LinkedList<T>{
    head: Option<Box<Node<T>>>,
}
struct Stack<T>{
    contract: LinkedList<T>,
    len: usize,
}

impl<T: std::fmt::Debug> LinkedList<T> { // Implementation for LinkedList
    fn new() -> Self{
        Self {
            head: None
        }
    }
    fn push_front(&mut self , val : T)   {
        let node= Node{value:val , next : self.head.take()};
        self.head = Some(Box::new(node));
    }
    fn pop_front(&mut self) -> std::option::Option<T> {
        let node = self.head.take();
        match node {
            Some(node) => {
                let val = Some(node.value);
                self.head = node.next;
                return val;
            },
            None => {
                return None;
            }
        }
    }
    fn is_empty(&self) -> bool {
        !self.head.is_some()
    }
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    fn print_list(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current{
            print!("{:?} " , node.value );
            current=node.next.as_ref();
        }
        println!("None");
    }

}

impl<T: std::fmt::Debug> Stack<T> { //Implementation for stack
    fn new() -> Self {
        Self {
            contract : LinkedList::new(),
            len : 0,
        }
    }
    fn push(&mut self, val : T) {
        self.contract.push_front(val);
        self.len += 1 ;
    }
    fn pop(&mut self) -> Option<T>{
        if self.is_empty(){
            return None;
        }
        else{
            self.len -= 1;
            self.contract.pop_front()
        }
    }
    fn peek(&self) -> Option<&T> {
        self.contract.peek()
    }
    fn size(&self) -> usize{
        self.len
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}


impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut current = self.contract.head.as_ref();
        let mut first = true;

        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", node.value)?;
            first = false;
            current = node.next.as_ref();
        }

        write!(f, "]")
    }
}




fn main() {
    let mut list = LinkedList::new(); //Linked List tests
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    println!("{:?}", list.pop_front());
    println!("{:?}", list.peek());
    let mut stack = Stack::new(); // Stack tests
    println!("Empty : {}" , stack.is_empty());
    stack.push(20);
    stack.push(30);
    stack.push(100);
    println!("Length of stack is {}",stack.size());
    println!("Empty : {}" , stack.is_empty());
    println!("{:?}" , stack.pop());
    println!("{:?}" , stack.peek());
    println!("{}" , stack);
}
