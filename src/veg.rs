
pub mod vegg {
    use std::ops::Deref;
    use crate::veg::vegg::VegState::*;


    #[derive(Debug, Clone , PartialEq)]
    pub struct Node<T: Copy + PartialEq> {

        pub next: VegState<T>,

        pub index: isize,

        pub value: T

    }

    #[derive(Debug, PartialEq)]
    pub struct Vegie<T: Copy + PartialEq> {

        pub len: isize,

        pub head:VegState<T>

    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum VegState<T: Copy + PartialEq> {

        End,

        PNext(Box<Node<T>>)

    }

    fn index_change<T: Copy + PartialEq + std::fmt::Debug>(by: isize, mut buffer: &mut Box<Node<T>>) {

        loop {

            buffer.index += by;

            buffer = match buffer.next {
                End => {break}
                PNext(ref mut h) => {h}
            };


        }


    }

    impl <T: Copy + PartialEq + std::fmt::Debug> Node<T> {

        fn rec_push(&mut self, mut value: Box<Node<T>>, len:  isize) {

            if self.index == len {

                match &mut value.next {
                    End => {}
                    PNext(ref mut t) => {index_change(1, t)}
                };

                self.next = PNext(value);

            }

            else {

                match self.next {
                    End => {panic!("index out of bounds")}
                    PNext(ref mut t) => {t.rec_push(value, len)}
                }

            }



        }

        fn rec_pop(&mut self, len:  isize) -> T{

            if self.index == len {

                match self.next {
                    End => {}
                    PNext(ref mut t) => {index_change(-1, t)}
                };

                let ne = match &self.next {
                    End => {panic!("smthng went wrong")},
                    PNext(t) => {
                        (t.next.clone(), t.value)
                    }
                };

                self.next = ne.0.clone();

                return ne.1;

            } else { match self.next {
                End => {panic!("smthng went wrong")},
                PNext(ref mut t) => {t.rec_pop(len)}
            }}

        }

        fn rec_update(&mut self, value: T, index:  isize) {

            if self.index == index {
                self.value = value;
                return;
            }
            else { match self.next {
                End => {panic!("index out of bounds")}
                PNext(ref mut t) => {t.rec_update(value, index)}
            }}

        }


    }

    impl<T: Copy + std::fmt::Debug + PartialEq> Vegie<T> {

        pub fn new() -> Vegie<T> {

            Vegie {len:0, head: End}

        }

        pub fn fetch(&self, index: isize) -> Node<T> {

            if index > self.len {

                panic!("index out of bounds")

            }

            let mut buffer = match &self.head {
                End => {panic!("index out of bounds")}
                PNext(t) => {*t.clone()}
            };



            for _ in 0 .. index{

                buffer = match buffer.next {
                    End => {panic!("index out of bounds")}
                    PNext(t) => {*t}
                }

            };

            return buffer;



        }

        pub fn push(&mut self, value: T) {

            self.insert(value, self.len)

        }

        pub fn pop(&mut self) -> T {

            let buf = self.fetch(self.len - 1).value;

            self.delete(self.len);

            return buf

            }



        pub fn update(&mut self, value: T, index: isize) {

            match self.head {
                End => {panic!("index out of bounds") },
                PNext(ref mut t) => {t.rec_update(value, index + 1)}}
        }

        pub fn delete(&mut self, index: isize) {
            if self.len == 0 {panic!("empty stack")};

            self.len -= 1;

            if index == 0 {

                let ne = match &mut self.head {
                    End => {panic!("smthng went wrong")},
                    PNext(ref mut t) => {

                        index_change(-1, t);

                        match &self.head {
                            End => {panic!("smthng went wrong")}
                            PNext(o) => {o.next.clone()}
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
                End => {panic!("smthng went wrong")},
                PNext(ref mut t) => {t.rec_pop(index)}};
        }

        pub fn insert(&mut self, value: T, index: isize) {

            let ne = match index {
                0 => {self.head.clone()}

                _ => {self.fetch(index - 1).next}

            };


            let mut new_node = Box::new(Node {
                next: ne,

                index: index + 1,

                value

            });

            self.len += 1;

            if index == 0{

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
                End => {panic!("smthng went wrong")}
                PNext(ref mut t) => {t.rec_push(new_node, index); }

            };

        }

    }
}

#[cfg(test)]
pub mod tests {
    use crate::veg::vegg::*;
    use crate::veg::vegg::VegState::*;

    #[test]
    fn fetch() {

        let mut v = Vegie { len: 2, head: PNext(Box::new(Node {
            next: PNext(Box::new(Node {
                next: End,
                index: 2,
                value: 6,
            })),
            index: 1,
            value: 5,
        })) };



        assert_eq!(5, v.fetch(0).value);
        assert_eq!(6, v.fetch(1).value);


    }


    #[test]
    fn push_pop() {

        let mut v = Vegie::new();
        
        v.push(5);
        
        assert_eq!(v, Vegie { len: 1, head: PNext(Box::new(Node {
            next: End,
            index: 1,
            value: 5,
        })) });

        let r = v.pop();

        assert_eq!(r, 5);
        assert_eq!(v, Vegie { len: 0, head: End })

    }



    #[test]
    fn update() {

        let mut v = Vegie::new();

        v.push(5);
        v.push(5);



        v.update(4, 0);
        v.update(3, 1);

        assert_eq!(4, v.fetch(0).value);
        assert_eq!(3, v.fetch(1).value);

    }

    #[test]
    fn delete() {

        let mut v = Vegie::new();

        v.push(5);
        v.push(4);
        v.push(3);

        v.delete(1);


    }

    #[test]
    fn insert() {

        let mut v = Vegie::new();

        v.push(5);
        v.push(4);

        v.insert(6, 0);

        dbg!(&v);


    }



}





