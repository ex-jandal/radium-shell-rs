use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    pub size: usize,
}

impl<T: std::clone::Clone + std::fmt::Display> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: value,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
        self.size += 1;

        if self.size > 1000 {
            self.pop_front();
        }
    }

    fn pop_front(&mut self) {
        if let Some(old_head) = self.head.take() {
            let next_node = old_head.borrow_mut().next.take();

            match next_node {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.size -= 1;
        }
    }

    pub fn display_all(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!("nullptr");
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut current = self.head.clone();
        let mut result = Vec::<T>::new();

        while let Some(node) = current {
            result.push(node.borrow_mut().data.clone());
            current = node.borrow().next.clone();
        }

        result
    }
    pub fn purge(&mut self) {
        while self.head.is_some() {
            self.pop_front();
        }
    }
}
