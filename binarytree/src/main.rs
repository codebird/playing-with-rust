use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
    height: i8,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn get_height(&self) -> i8 {
        match self.0 {
            None => 0,
            Some(ref bd) => bd.height,
        }
    }
    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.height = 1 + std::cmp::max(t.left.get_height(), t.right.get_height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }
    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}
impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.height = 1 + std::cmp::max(res.left.get_height(), res.right.get_height());
        res
    }
    pub fn rot_right(mut self) -> Box<Self> {
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.height = 1 + std::cmp::max(res.left.get_height(), res.right.get_height());
        res
    }
}
impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, depth: usize) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(depth + 1);
            let mut spc = String::new();
            for _ in 0..depth {
                spc.push('.');
            }
            println!("{}:{} {:?}", bd.height, spc, bd.data);
            bd.right.print_lfirst(depth + 1);
        }
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            None => {
                self.0 = Some(Box::new(BinData {
                    data: data,
                    height: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
            Some(ref mut node) => {
                if data < node.data {
                    node.left.add_sorted(data);
                    if node.left.get_height() - node.right.get_height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    node.right.add_sorted(data);
                    if node.right.get_height() - node.left.get_height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
        };
        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

fn main() {
    let mut tree = BinTree::new();
    tree.add_sorted(4);
    tree.add_sorted(5);
    tree.add_sorted(6);
    tree.add_sorted(10);
    tree.add_sorted(1);
    tree.add_sorted(94);
    tree.add_sorted(54);
    tree.add_sorted(3);
    for i in 0..100000 {
        tree.add_sorted(i);
    }
    tree.print_lfirst(0);
}
