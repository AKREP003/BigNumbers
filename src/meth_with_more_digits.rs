mod dig {
    use std::cmp::Ordering::*;
    use std::ops::*;
    use crate::veg::vegg::{Vegie, VegState};


    #[derive(Debug, Clone)]
    pub struct IDig {
        rpoint: isize,
        sign:bool,
        body: Vegie<u64>
    }

    pub fn p(n1: f64, n2: isize) -> f64 {
        if n1 == 0.0 { return 1.0; } //lim

        let mut buffer = n1.clone();

        if n2 > 0 {
            for _ in 1..n2 {
                buffer = buffer * n1;
            }
        } else {
            for _ in -1..(n2 * -1) {
                buffer = buffer / n1;
            }
        }

        return buffer;
    }

    pub fn d(x:f64, y:i32) -> f64 {
        ((x / p(8.0,y.try_into().unwrap())) % 8.0).floor()
    }

    impl IDig {

        pub fn new(mut from:Vegie<u64>, rpoint:isize, sign: bool) -> Self{

            let l:u64 = 9223372036854775807;

            for i in from.clone() {

                if i > l {
                    panic!("E")
                }

            }



            IDig {
                rpoint,
                body: from,
                sign
            }


        }

    }

    fn resize(n1: &IDig, n2:&IDig) -> (Vegie<u64>, Vegie<u64>, isize) {

        let mut r1 = n1.body.clone();

        let mut r2 = n2.body.clone();

        let mut rp:isize;

        if r1.len - n1.rpoint > r2.len - n2.rpoint {

            for _ in 0 .. (r1.len - n1.rpoint) - (r2.len - n2.rpoint) {

                r2.insert(0, 0);

            }

        } else {

            for _ in 0 .. (r2.len - n2.rpoint) - (r1.len - n1.rpoint){

                r1.insert(0, 0);

            }

        }

        if n1.rpoint >  n2.rpoint {

            rp = n1.rpoint.clone();

            for _ in 0 .. (n1.rpoint) - (n2.rpoint) {

                r2.push(0);

            }

        } else {

            rp = n2.rpoint.clone();

            for _ in 0 .. (n2.rpoint) - (n1.rpoint){

                r1.push( 0);

            }

        }

        return (r1, r2, rp)

    }


    impl PartialEq<Self> for IDig {
        fn eq(&self, other: &Self) -> bool {

            return !{self < other} && !{other < self}

        }
    }

    impl PartialOrd for IDig {

        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

            if self.sign != other.sign {
                return if self.sign { //x  (-y)

                    Some(Greater)
                } else { //-x  (y)

                    Some(Less)
                }

            }

            else {

                if !self.sign {

                    let mut b1 = self.clone();
                    let mut b2 = other.clone();

                    b1.sign = true;
                    b2.sign = true;

                    return match {b1 < b2} {
                        true => {Some(Greater)}
                        false => {Some(Less)}
                    }

                }

                let siz = resize(&self, &other);

                let mut e1 = match siz.0.head {
                    VegState::End => {return Some(Equal)}
                    VegState::PNext(t) => {t}
                };

                let mut e2 = match siz.1.head {
                    VegState::End => {return Some(Equal)}
                    VegState::PNext(t) => {t}
                };

                for _ in 0 .. siz.0.len {

                    if e1.value != e2.value {
                        return if e1.value < e2.value {
                            Some(Less)
                        } else { Some(Greater) }

                    } else {

                        e1 = match e1.next {
                            VegState::End => {break}
                            VegState::PNext(t) => {t}
                        };

                        e2 = match e2.next {
                            VegState::End => {break}
                            VegState::PNext(t) => {t}
                        };


                    }



                }


            }


            Some(Equal)
        }

        
    }

    impl Sub for IDig {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {

            if self.sign != rhs.sign {

                let mut buuf;

                buuf = rhs.clone();

                buuf.sign = !buuf.sign;

                return self + buuf;


            }

            if self < rhs {


                let mut br = rhs - self;

                br.sign = !br.sign;

                return br

            }


            let siz = resize(&self, &rhs);

            let mut r1 = siz.0;

            let mut r2 = siz.1;

            let mut rp = siz.2;



            let mut hand = IDig::new(Vegie::new(vec![]), 0, true);

            for i in 0 .. r1.len {

                let v1 = r1.fetch(r1.len - i - 1).value; //I am not the lost one. Vegie is not bi-directional.

                let v2 = r2.fetch(r1.len - i - 1).value;

                if v1 < v2 {

                    let k:u64 = v1 + 9223372036854775807; //shift the offset

                    r1.update(k - v2, r1.len - i - 1);

                    let mut v:Vegie<u64> = Vegie::new(vec![0; (i) as usize]);

                    v.insert(1, 0);


                    let b = IDig::new(v, rp - 1, true);


                    hand = hand + b


                } else {


                    r1.update(v1 - v2, r1.len - i - 1);


                }

            }


            if hand.body.len != 0 {

                return  IDig::new(r1, rp, true) - hand

            }

            return  IDig::new(r1, rp, true)


        }
    }

    impl Add for IDig {

        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {

            if self.sign != rhs.sign {

                let mut buuf;

                return if !self.sign {
                    buuf = self.clone();

                    buuf.sign = true;

                    rhs - buuf
                } else {
                    buuf = rhs.clone();

                    buuf.sign = true;

                    self - buuf
                }


            }

            let sig = self.sign;

            let siz = resize(&self, &rhs);

            let mut r1 = siz.0;

            let mut r2 = siz.1;

            let mut rp = siz.2;



            let mut hand = IDig::new(Vegie::new(vec![]), 0, sig);

            for i in 0 .. r1.len {

                let swh =  r1.fetch(r1.len - i - 1).value + r2.fetch(r1.len - i - 1).value;

                if swh > 9223372036854775807 {

                    r1.update(swh - 9223372036854775808, r1.len - i - 1);

                    let mut v:Vegie<u64> = Vegie::new(vec![0; (i + 1) as usize]);

                    v.insert(1, 0 );

                    let b = IDig::new(v, rp, sig);

                    hand = hand + b; //there is no fucking way to test it


                }

                else {

                    r1.update(swh, r1.len - i - 1);

                }

            }

            if hand.body.len != 0 {

                dbg!(&hand);

                return  IDig::new(r1, rp, sig) + hand

            }

            return  IDig::new(r1, rp, sig)

        }

    }


}

#[cfg(test)]
pub mod tests {
    use crate::meth_with_more_digits::dig::*;
    use crate::Vegie;

    #[test]
    fn n() {

        let mut v1:Vegie<u64> = Vegie::new(vec![9223372036854775810,0]);

        let I = IDig::new(v1.clone(), 0, true);

        dbg!(I);

    }

    #[test]
    fn s()  {

        let mut v1:Vegie<u64> = Vegie::new(vec![3,0]);

        let mut v2:Vegie<u64> = Vegie::new(vec![1,0]);


        let I = IDig::new(v1.clone(), 0, true);


        let D = IDig::new(v2.clone(), 1, true);

        dbg!(I + D);

    }


    #[test]
    fn a() {

        let mut v1:Vegie<u64> = Vegie::new(vec![1,0]);

        let mut v2:Vegie<u64> = Vegie::new(vec![1]);


        let I = IDig::new(v1.clone(), 0, false);


        let D = IDig::new(v2.clone(), 0, true);

        dbg!(I - D);

    }

    #[test]
    fn cm() {

        let mut v1:Vegie<u64> = Vegie::new(vec![1]);

        let mut v2:Vegie<u64> = Vegie::new(vec![1]);


        let I = IDig::new(v1.clone(), 0, true);


        let D = IDig::new(v2.clone(), 0, true);

        dbg!(I == D);

    }


}