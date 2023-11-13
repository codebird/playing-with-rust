use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Write};
use std::rc::Rc;
type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

#[derive(Debug)]
pub struct SkipList<T: PartialOrd>(Vec<SkipNode<T>>);

impl<T: PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    pub fn insert(&mut self, data: T) {
        if self.0.len() == 0 {
            self.0.push(SkipNode::new(data));
            return;
        }
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(ch) = self.0[i].insert(data) {
                    self.loop_up(ch, i + 1);
                };
                return;
            }
        }
        let mut nn = SkipNode::new(data);
        std::mem::swap(&mut nn, &mut self.0[0]);
        let res = rcc(nn);
        self.0[0].right = Some(res.clone());
        self.loop_up(res, 1);
    }

    pub fn loop_up(&mut self, ch: Rcc<SkipNode<T>>, n: usize) {
        if rand::random::<bool>() == true {
            return;
        }
        let dt = ch.borrow().data.clone();
        let mut nn = SkipNode {
            right: None,
            down: Some(ch),
            data: dt,
        };
        if n >= self.0.len() {
            self.0.push(nn);
            return;
        }

        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rcc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n + 1);
    }
}

impl<T: Debug + PartialOrd> fmt::Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "Skiplist<Empty>");
        }
        for sn in &self.0 {
            write!(f, "\n")?;
            sn.print_row(f)?;
        }
        Ok(())
    }
}

impl<T: Debug + PartialOrd> SkipNode<T> {
    pub fn print_row<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref rt) = self.right {
            write!(w, ",")?;
            rt.borrow().print_row(w)?;
        }
        Ok(())
    }
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, t: T) -> Option<Rcc<SkipNode<T>>> {
        if let Some(ref mut rt) = self.right {
            if t > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(t);
            }
        }

        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(t) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        let dt = child.borrow().data.clone();
                        let nn = SkipNode {
                            right: self.right.take(),
                            data: dt,
                            down: Some(child),
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    }
                    false => None,
                },
                None => None,
            };
        }

        let mut nn = SkipNode::new(t);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }
}

fn main() {
    let mut snode = SkipList::new();
    snode.insert(4);
    snode.insert(4);
    snode.insert(6);
    snode.insert(77);
    snode.insert(84);
    snode.insert(23);
    snode.insert(4);
    snode.insert(4);
    snode.insert(6);
    snode.insert(77);
    snode.insert(84);
    snode.insert(23);
    snode.insert(1);
    println!("{}", snode);
}
