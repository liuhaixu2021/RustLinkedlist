#[derive(Debug)]

struct Node{
     val:i32,
     next:Option<Box<Node>>,
}
//box only
//new node
impl Node{
     fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, next: None })
    }

    fn link(&mut self,node:Box<Node>){
        self.next=Some(node);
    }
    fn changevalue(&mut self,newvalue:i32){
        self.val=newvalue;
    }
}
//new list
#[derive(Debug)]

struct Linklist{
    head:Option<Box<Node>>,
}

impl Linklist{
fn new() -> Linklist{
    Linklist{head:None}
}
fn printnode(&self){
    println!("{:?}",self)
}
fn push_back(&mut self, value: i32) {
    let new_node = Node::new(value);
    match self.head.as_mut() {
        None => self.head = Some(new_node),
        Some(mut curr) => {
            while curr.next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }//若头结点不为空时，只要下一个节点是new_node
            curr.link(new_node);
        }
    }
}
fn change(&mut self, value: i32, newvalue: i32) -> bool{
    match self.head.as_mut() {
        None => false,// 若链表为空;
        Some(mut curr) => {
            //若插入值不等于头结点，则找下一个节点
                while curr.next.is_some() && curr.val != value {
                    curr = curr.next.as_mut().unwrap();
                }//往后移

                // 如果插入的位置不为尾节点之后；取出下一个节点并获取其所有权并link到新节点之后；
                if value == curr.val {//判定是否为尾节点
                    curr.changevalue(newvalue);
                    true
                }
               else{
                   false
               }
            
        }
    }
}
fn insert(&mut self, value: i32, newvalue: i32) {
    let mut new_node = Node::new(newvalue);
    match self.head.as_mut() {
        None => self.head = Some(new_node), // 若链表为空;
        Some(mut curr) => {
            //若插入值不等于头结点，则找下一个节点
                while curr.next.is_some() && curr.val != value {
                    curr = curr.next.as_mut().unwrap();
                }

                // 如果插入的位置不为尾节点之后；取出下一个节点并获取其所有权并link到新节点之后；
                if let Some(node) = curr.next.take() {//判定是否为尾节点
                    new_node.link(node);
                }
                // 当前节点的next指向新节点；
                curr.next = Some(new_node);
            
        }
    }
}
fn delete(&mut self, value: i32) -> bool {
    match self.head.as_mut() {
        None => false, // 链表为空；
        Some(mut curr) => {
            // 待删除节点为头节点的情况; 此时 curr == head;
            if curr.val == value {
                self.head = self.head.take().unwrap().next;
                true
            } else {
                // 从头节点的下一个节点开始寻找待删除节点，直到尾节点或找到为止；
                while curr.next.is_some() && curr.next.as_ref().unwrap().val != value {
                    curr = curr.next.as_mut().unwrap();
                }

                // 当前curr的下一个节点的值等于value，或者curr为尾节点；
                match curr.next.take() {
                    None => false, // curr已经是尾节点，这时相当于没有找到符合条件的节点;
                    Some(node) => {
                        // 获得下一个节点，并将其next的值的所有权转移到curr.next，删除成功；
                        curr.next = node.next;
                        true
                    }
                }
            }
        }
    }
}

}
//插入

//1 修改特定位置的节点的值
//2 在特定节点后插入新节点
//3 删除特定节点之后的节点
fn main() {
    let mut list = Linklist::new();
    for i in 1..6 {
        list.push_back(i);
    }
    list.printnode();
    println!();
    list.insert(3,10);
    list.printnode();
    println!();
    list.delete(5);
    list.printnode();
    println!();
    list.change(3,10);
    list.printnode();
}
