#![allow(non_snake_case)]


pub mod vegg {
    use std::fmt::{Display, Formatter, Result};

    use crate::vegg::VegState::{End, PNext};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Node<T: Copy + PartialEq> {
        pub next: VegState<T>,

        pub index: isize,

        pub value: T,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vegie<T: Copy + PartialEq> {
        pub len: isize,

        pub head: VegState<T>,

        pub curry: isize,
    }

    pub struct Iter<T: Copy + PartialEq> {
        pub current: VegState<T>,
        pub prevous: VegState<T>,
        pub len: isize,
        pub index: isize,

    }

    impl<T: Copy + PartialEq + std::fmt::Debug> Iterator for Iter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {

            let buffer = match self.current {
                End => { return None; }
                PNext(ref h) => { h.clone() }
            };

            if self.index < self.len {

                self.index += 1;
                self.prevous = self.current.clone();

                self.current = match self.current {
                    End => { panic!("smthng went wrong") }
                    PNext(ref h) => { h.next.clone() }
                };


            }

            return Some(buffer.value);

        }
    }

    impl<T: Copy + PartialEq + std::fmt::Debug> Iter<T> {

        pub fn go_back(&mut self) {
            self.current =  self.prevous.clone();
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum VegState<T: Copy + PartialEq> {
        End,

        PNext(Box<Node<T>>),
    }

    fn index_change<T: Copy + PartialEq + std::fmt::Debug>(by: isize, mut buffer: &mut Box<Node<T>>) {
        loop {
            buffer.index += by;

            buffer = match buffer.next {
                End => { break; }
                PNext(ref mut h) => { h }
            };
        }
    }

    impl<T: Copy + PartialEq + std::fmt::Debug> Node<T> {
        fn rec_push(&mut self, mut value: Box<Node<T>>, len: isize) {
            if self.index == len {
                match &mut value.next {
                    End => {}
                    PNext(ref mut t) => { index_change(1, t) }
                };

                self.next = PNext(value);
            } else {
                match self.next {
                    End => { panic!("index out of bounds") }
                    PNext(ref mut t) => { t.rec_push(value, len) }
                }
            }
        }

        fn rec_pop(&mut self, len: isize) -> T {
            if self.index == len {
                match self.next {
                    End => {}
                    PNext(ref mut t) => { index_change(-1, t) }
                };

                let ne = match &self.next {
                    End => { panic!("smthng went wrong") }
                    PNext(t) => {
                        (t.next.clone(), t.value)
                    }
                };

                self.next = ne.0.clone();

                return ne.1;
            } else {
                match self.next {
                    End => { panic!("smthng went wrong") }
                    PNext(ref mut t) => { t.rec_pop(len) }
                }
            }
        }

        fn rec_update(&mut self, value: T, index: isize) {
            if self.index == index {
                self.value = value;
                return;
            } else {
                match self.next {
                    End => { panic!("index out of bounds") }
                    PNext(ref mut t) => { t.rec_update(value, index) }
                }
            }
        }
    }


    impl<T: Copy + std::fmt::Debug + PartialEq> Vegie<T> {
        pub fn new(from: Vec<T>) -> Vegie<T> {
            let mut b = Vegie { len: 0, head: End, curry: 0 };

            for i in from {
                b.push(i)
            };

            b
        }

        pub fn extend(&self, to_extend: Vegie<T>) -> Vegie<T> {
            let mut buffer = self.clone();

            for i in to_extend.initiate_iter() {
                buffer.push(i)
            }

            buffer
        }

        pub fn fetch(&self, index: isize) -> Node<T> {
            if index > self.len || index < 0 {
                panic!("index out of bounds")
            }

            let mut buffer = match &self.head {
                End => { panic!("index out of bounds") }
                PNext(t) => { *t.clone() }
            };


            for _ in 0..index {
                buffer = match buffer.next {
                    End => { panic!("index out of bounds") }
                    PNext(t) => { *t }
                }
            };

            return buffer;
        }

        pub fn slice(&self, from: isize, to: isize) -> Vegie<T> {
            let mut new_slice = Vegie::new(vec![]);

            if from > self.len.try_into().unwrap() || from < 0 || to > self.len.try_into().unwrap() || to < 0 || from > to {
                panic!("index out of bounds")
            }

            let mut buffer = match &self.head {
                End => { panic!("index out of bounds") }
                PNext(t) => { *t.clone() }
            };


            for _ in 0..from {
                buffer = match buffer.next {
                    End => { panic!("index out of bounds") }
                    PNext(t) => { *t }
                }
            };

            new_slice.push(buffer.value);

            for _ in 0..(to - from) {
                buffer = match buffer.next {
                    End => { panic!("index out of bounds") }
                    PNext(t) => { *t }
                };

                new_slice.push(buffer.value);
            };

            return new_slice;
        }


        pub fn push(&mut self, value: T) {
            self.insert(value, self.len)
        }

        pub fn pop(&mut self) -> T {
            let buf = self.fetch(self.len - 1).value;

            self.delete(self.len - 1);

            return buf;
        }

        pub fn initiate_iter(&self) -> Iter<T> {
            Iter {
                current: self.head.clone(),
                prevous: End,
                len: self.len,
                index: 0,
            }
        }

        pub fn update(&mut self, value: T, index: isize) {
            match self.head {
                End => { panic!("index out of bounds") }
                PNext(ref mut t) => { t.rec_update(value, index + 1) }
            }
        }

        pub fn delete(&mut self, index: isize) {
            if self.len == 0 { panic!("empty stack") };

            self.len -= 1;

            if index == 0 {
                let ne = match &mut self.head {
                    End => { panic!("smthng went wrong") }
                    PNext(ref mut t) => {
                        index_change(-1, t);

                        match &self.head {
                            End => { panic!("smthng went wrong") }
                            PNext(o) => { o.next.clone() }
                        }
                    }
                };

                self.head = ne.clone();


                return;
            };

            if self.len == 0 {
                self.head = End;
                return;
            }

            match self.head {
                End => { panic!("smthng went wrong") }
                PNext(ref mut t) => { t.rec_pop(index) }
            };
        }

        pub fn insert(&mut self, value: T, index: isize) {
            let ne = match index {
                0 => { self.head.clone() }

                _ => { self.fetch(index - 1).next }
            };


            let mut new_node = Box::new(Node {
                next: ne,

                index: index + 1,

                value,
            });

            self.len += 1;

            if index == 0 {
                match &mut new_node.next {
                    End => {}
                    PNext(ref mut t) => {
                        index_change(1, t);
                    }
                };

                self.head = PNext(new_node);

                return;
            }


            match self.head {
                End => { panic!("smthng went wrong") }
                PNext(ref mut t) => { t.rec_push(new_node, index); }
            };
        }
    }


    impl<T: Display + Copy + PartialEq + std::fmt::Debug> Display for Vegie<T> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let mut r = String::new();

            for i in self.initiate_iter() {
                r = r + &format!("{}", i).to_string();
            }

            write!(f, "{}", r)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::veg::vegg::*;
    use crate::veg::vegg::VegState::*;
    use crate::vegg::Vegie;

    #[test]
    fn fetch() {}


    #[test]
    fn push_pop() {
        let mut v = Vegie::new(vec![]);

        v.push(5);

        let r = v.pop();

        assert_eq!(r, 5);
    }


    #[test]
    fn update() {
        let mut v = Vegie::new(vec![]);

        v.push(5);
        v.push(5);


        v.update(4, 0);
        v.update(3, 1);

        assert_eq!(4, v.fetch(0).value);
        assert_eq!(3, v.fetch(1).value);
    }

    #[test]
    fn delete() {
        let mut v = Vegie::new(vec![]);

        v.push(5);
        v.push(4);
        v.push(3);

        v.delete(1);
    }

    #[test]
    fn insert() {
        let mut v = Vegie::new(vec![]);

        v.push(5);
        v.push(4);

        v.insert(6, 0);

        dbg!(&v);
    }

    #[test]
    fn itr() {
        let mut v = Vegie::new(vec![]);

        v.push(1);
        v.push(2);

        for i in v.clone() {
            for j in v.clone() {
                dbg!(i, j);
            }
        }
    }

    #[test]
    fn ext() {
        let mut v = Vegie::new(vec![1]);

        let mut v2 = Vegie::new(vec![2]);

        v.extend(v2);

        dbg!(v);
    }

    #[test]
    fn dis() {
        let mut v = Vegie::new(vec![1]);

        println!("{}", v);
    }
}





