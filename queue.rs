struct Node<T> {
    value : T,
    next: Option<Box<Node<T>>>
}
struct Queue<T>{
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>
}
impl<T : Clone> Queue<T>{
    fn new() -> Self {
        Self {
            head : None,
            tail : None,
        }
    }
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    fn queue(&mut self, val: T) {
            let mut new_node = Box::new(Node {
                value: val,
                next: None,
            });
            let raw: *mut _ = &mut *new_node;
            if self.is_empty() {
                self.head = Some(new_node);
                self.tail = Some(raw);
            } else {
                unsafe {
                    (*self.tail.unwrap()).next = Some(new_node);
                    self.tail = Some(raw);

                }

            }

    }
    fn unqueue(&mut self) -> Option<T>{
        let node = self.head.take();
        match node {
            Some(node) => {
                let val = Some(node.value);
                if node.next.is_none(){
                    self.head = None;
                    self.tail = None;
                }
                else{
                    self.head = node.next;
                }
                return val;
            },
            None => {
                return None;
            }
        }
    }



}


fn main(){}
