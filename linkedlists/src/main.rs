use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Option<Box<LinkedList<T>>>)>);

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
}
