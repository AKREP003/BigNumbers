#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]


mod dig {
    use std::cmp::{max};
    use std::cmp::Ordering::*;
    use std::fmt::{Display, Formatter, Result};
    use std::ops::*;
    use Veggies::vegg::{Vegie};
    static BASE:u64 = 10;
    //9223372036854775807

    static MAX_NEW_DIGIT:usize = 5;


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
        ((x / p(10.0,y.try_into().unwrap())) % 10.0).floor()
    }

    impl IDig {

        pub fn new(n :i64) -> Self{

            return IDig::from(Vegie::new(vec![n.abs().try_into().unwrap()]), 0, n >= 0)

        }

        pub fn from(mut from:Vegie<u64>, rpoint:isize, sign: bool) -> Self{

            for i in from.clone() {

                if i >= BASE {
                    dbg!(i);
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

                r2.push( 0);

            }

        } else {

            for _ in 0 .. (r2.len - n2.rpoint) - (r1.len - n1.rpoint){

                r1.push( 0);

            }

        }

        if n1.rpoint >  n2.rpoint {

            rp = n1.rpoint.clone();

            for _ in 0 .. (n1.rpoint) - (n2.rpoint) {

                r2.insert(0,0 );

            }

        } else {

            rp = n2.rpoint.clone();

            for _ in 0 .. (n2.rpoint) - (n1.rpoint){

                r1.insert( 0, 0);

            }

        }

        return (r1, r2, rp)

    }


    impl PartialEq<Self> for IDig {
        fn eq(&self, other: &Self) -> bool {

            if !(self.sign == other.sign) {

                let (mut n1, mut n2, _) = resize(&self, &IDig::new(0));

                if n1 == n2 {

                    let (n3, n4, _) = resize(&self, &IDig::new(0));

                    return n3 == n4;
                }

                return false

            } else {
                let (n1, n2, _) = resize(&self, other);

                return n2 == n1
            }

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

                    return match {b1 > b2} {
                        true => {Some(Greater)}
                        false => {Some(Less)}
                    }

                }

                let siz = resize(&self, &other);

                for i in 0 .. siz.1.len {

                    let v1 = siz.0.fetch(siz.0.len - i - 1).value;
                    let v2 = siz.1.fetch(siz.0.len - i - 1).value;

                    if v1 > v2 {
                        return Some(Greater)
                    }
                    if v1 < v2 {
                        return Some(Less)
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



            let mut hand = IDig::from(Vegie::new(vec![]), 0, true);

            for i in 0 .. r1.len {

                let v1 = r1.fetch(r1.len - i - 1).value; //I am not the lost one. Vegie is not bi-directional.

                let v2 = r2.fetch(r1.len - i - 1).value;

                if v1 < v2 {

                    let k:u64 = v1 + BASE; //shift the offset

                    r1.update(k - v2, r1.len - i - 1);

                    let mut v:Vegie<u64> = Vegie::new(vec![0; (i) as usize]);

                    v.insert(1, 0);


                    let b = IDig::from(v, rp - 1, true);


                    hand = hand + b


                } else {


                    r1.update(v1 - v2, r1.len - i - 1);


                }

            }


            if hand.body.len != 0 {

                return  IDig::from(r1, rp, true) - hand

            }

            return  IDig::from(r1, rp, true)


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



            let mut hand = IDig::from(Vegie::new(vec![]), 0, sig);

            for i in 0 .. r1.len {

                let swh =  r1.fetch(r1.len - i - 1).value + r2.fetch(r1.len - i - 1).value;

                if swh >= BASE {

                    r1.update(swh - BASE, r1.len - i - 1);

                    let mut v:Vegie<u64> = Vegie::new(vec![0; (i + 1) as usize]);

                    v.insert(1, 0 );

                    let b = IDig::from(v, rp, sig);

                    hand = hand + b; //there is no fucking way to test it


                }

                else {

                    r1.update(swh, r1.len - i - 1);

                }

            }

            if hand.body.len != 0 {


                return  IDig::from(r1, rp, sig) + hand

            }

            return  IDig::from(r1, rp, sig)

        }

    }


    impl Div for IDig {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {

            if rhs == IDig::new(0){

                panic!("Division by 0")

            }

            if self == IDig::new(0){

                return self

            }


            let sig = if self.sign == rhs.sign {self.sign} else { false };

            let mut buf = self.clone();

            let buf_point = buf.rpoint;

            let mut buf_rhs = rhs.clone();

            let rhs_point = rhs.rpoint;

            buf_rhs.sign = true;

            buf.sign = true;

            buf_rhs.rpoint = 0;

            buf.rpoint = 0;
            let mut result_body = IDig::from(Vegie::new(vec![]), 0, true);

            let empty =IDig::from(Vegie::new(vec![0]), 0, true);

            let mut extra_digit_added = 0;

            //while buf.body != empty
            //for _ in 0 .. 5

            while buf != empty  {

                if buf < buf_rhs {

                    if extra_digit_added >= MAX_NEW_DIGIT {

                        break

                    }
                    else {

                        extra_digit_added += 1;

                        buf.body.insert(0, 0);

                    }

                }

                let mut b =  Vegie::new(vec![]);

                result_body = result_body + IDig::from(b.extend(Vegie::new(vec![1])), extra_digit_added as isize, true);

                buf = buf - IDig::from(rhs.body.clone(), 0, true);


            }

            let to_add = buf_point - rhs_point;

            if to_add < 0 {

                panic!("increase the maximum additional digit number")

            }

            result_body.rpoint += to_add;

            result_body.sign = sig;

            return result_body

        }
    }

    impl Mul for IDig {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output { //Glory To Anatoly Karatsuba

            let zero = IDig::new(0);

            if rhs == zero || self == zero{ // I am something of an optimizer myself

                return zero

            };

            let sig = if self.sign == rhs.sign {self.sign} else { false };

            let mut buf = self.clone();



            return buf

        }
    }

    impl Display for IDig {

        fn fmt(&self, f: &mut Formatter) -> Result {

            let sign = match self.sign { true => "", false => "-"};

            let mut stringfied_body = self.body.to_string();

            while stringfied_body.len() != 0 && stringfied_body.chars().nth(stringfied_body.len() - 1 as usize).unwrap() == '0'{

                stringfied_body.pop();

            }

            let safe_rpoint = max(0, self.rpoint);



            for _ in 0 .. max(0, safe_rpoint - self.body.len){

                stringfied_body.push('0')

            }


            if safe_rpoint != 0 {

                stringfied_body.insert(safe_rpoint as usize, ".".parse().unwrap());
            };


            if self < &IDig::new(1) && self > &IDig::new(-1){
                stringfied_body.push('0');

            };

            write!(f, "{}{}", sign, stringfied_body.chars().rev().collect::<String>())
        }

    }


}

#[cfg(test)]
pub mod tests {
    use crate::dig::*;

    use Veggies::vegg::Vegie;



    #[test]
    fn n() {

        let mut v1:Vegie<u64> = Vegie::new(vec![9,0]);

        let I = IDig::from(v1.clone(), -1, true);

        println!("{}",I);

    }

    #[test]
    fn s()  {

        let mut v1:Vegie<u64> = Vegie::new(vec![3]);

        let mut v2:Vegie<u64> = Vegie::new(vec![2, 1, 1]);


        let I = IDig::from(v1.clone(), 1, true);


        let D = IDig::from(v2.clone(), 0, true);



        println!("{} + {} = {}",I.clone(), D.clone(), I + D);

    }


    #[test]
    fn a() {

        let mut v1:Vegie<u64> = Vegie::new(vec![6]);

        let mut v2:Vegie<u64> = Vegie::new(vec![0, 1]);


        let I = IDig::from(v1.clone(), 0, true);


        let D = IDig::from(v2.clone(), 0, true);

        println!("{} - {} = {}",I.clone(), D.clone(), I - D);

    }

    #[test]
    fn cm() {

        let mut v1:Vegie<u64> = Vegie::new(vec![2]);

        let mut v2:Vegie<u64> = Vegie::new(vec![0, 1]);


        let I = IDig::from(v1.clone(), 0, true);


        let D = IDig::from(v2.clone(), 0, true);

        dbg!(I < D);

    }

    #[test]
    fn dividy() {

        let mut v1:Vegie<u64> = Vegie::new(vec![1]);

        let mut v2:Vegie<u64> = Vegie::new(vec![3]);


        let I = IDig::from(v1.clone(), 0, true);


        let D = IDig::from(v2.clone(), 0, true);

        println!("{}", I / D);



    }

    #[test]
    fn multip() {

        let mut v1:Vegie<u64> = Vegie::new(vec![0, 2,0]);

        let mut v2:Vegie<u64> = Vegie::new(vec![3]);


        let I = IDig::from(v1, 0, true);


        let D = IDig::from(v2, 0, true);

        println!("{}", I + D);



    }

}
