use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Option<Box<LinkedList<T>>>)>);

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(old_first) => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(old_first.clone()),
                    prev: None,
                }));
                old_first.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            }
        }
    }
    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(old_last) => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(old_last.clone()),
                    next: None,
                }));
                let old_last_ref = old_last.upgrade().unwrap();
                let mut mut_old_last = old_last_ref.borrow_mut();
                self.last = Some(Rc::downgrade(&new_node));
                mut_old_last.next = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            }
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let d = self.0.take();
        self.0 = Some((data, Some(Box::new(LinkedList(d)))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, Some(ref mut child))) => child.push_back(data),
            Some((_, None)) => {
                self.0.take().unwrap().1 = Some(Box::new(LinkedList(Some((data, None)))))
            }
            _ => self.push_front(data),
        }
    }

    pub fn push_ordered(&mut self, data: T)
    where
        T: PartialOrd + std::fmt::Debug,
    {
        if self.0.is_none() {
            self.push_front(data);
        } else if self.0.as_ref().unwrap().0 > data {
            self.push_front(data);
        } else {
            match self.0 {
                Some((_, Some(ref mut child))) => child.push_ordered(data),
                Some((_, None)) => {
                    self.0.take().unwrap().1 = Some(Box::new(LinkedList(Some((data, None)))))
                }
                _ => self.push_front(data),
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    list.push_front(0);
    list.push_back(4);
    list.push_ordered(3);
    println!("{:?}", list);

    let mut db_list = DbList::new();
    db_list.push_front(1);
    db_list.push_back(2);
    db_list.push_front(0);
    db_list.push_back(4);
    db_list.push_front(3);
    println!("{:?}", db_list);
}
