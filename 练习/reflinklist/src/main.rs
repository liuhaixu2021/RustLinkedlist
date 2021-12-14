use std::cell::RefCell;
use std::rc::Rc;
type LinkNode = Option<Rc<RefCell<Node>>>;
#[derive(Debug)]
struct Node {
    val: i32,
    next: LinkNode,
}

impl Node {
    fn new(value:i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val: value,
            next: None,
        }))
    }
}

#[derive(Debug)]
struct Linklist {
    head: LinkNode,
    tail: LinkNode,
}

impl Linklist {
    fn new() -> Self {
        Linklist {
            head: None,
            tail: None,
        }
    }

    fn printnode(&self){
        println!("{:?}",self)
    }

    fn change(&mut self, value: i32, newvalue: i32) -> bool{
        match &self.head {
            None=> false,
            Some(node) => {
                let mut curr = Some(Rc::clone(&node));
                loop {
                    match curr {
                        None=>break,
                        Some(ptr) =>{
                            if ptr.borrow().val == value {
                                ptr.borrow_mut().val = newvalue;
                            }
                            
                            curr = match &ptr.borrow().next {
                                None => None,
                                Some(p) => Some(Rc::clone(&p))//去下一个节点
                            }
                        }                        
                    }
                }
                false
            }
        }
    }

    fn push_back(&mut self, value: i32) {
        let node = Node::new(value);
        match self.tail.take() {
            Some(curr) => curr.borrow_mut().next = Some(node.clone()),//尾部有值，尾部值的下一个值变成node的地址
            None => self.head = Some(node.clone()),//尾部为空，直接变成头结点
        }
        self.tail = Some(node);//头结点同时也是尾结点
    }

    fn insert(&mut self, value: i32, newvalue: i32) -> bool{
        let mut new_node = Node::new(newvalue);
        match &self.head {
            None=> false,
            Some(node) => {
                let mut curr = Some(Rc::clone(&node));
                loop {
                    match curr {
                        None=>break,
                        Some(ptr) =>{
                            
                            if ptr.borrow().val == value {
                                //new_node.borrow_mut().next=Some(Rc::clone(&ptr));
                                ptr.borrow_mut().next = Some(new_node.clone());//插入节点
                            }
                            curr = match &ptr.borrow().next {
                                None => None,
                                Some(p) => Some(Rc::clone(&p))
                            }
                          
                        }                        
                    }
                }
                false
            }
        }
    }


}

fn main() {
    let mut list = Linklist::new();
    for i in 1..6 {
        list.push_back(i);
    }
    list.printnode(); 
    list.change(3,10);
    list.printnode(); 
    list.insert(5,10);
    list.printnode(); 
}