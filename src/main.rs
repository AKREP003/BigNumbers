#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]


mod meth_with_more_digits;

use Veggies::vegg::*;
use crate::meth_with_more_digits::dig::*;

fn main() {

    non_optimised_digger()


}

fn digger() {

    let _zero = IDig::new(0);

    let mut W = IDig::new(0);

    //let mut buf = Vegie::new(vec![]);


    loop {

            W = W + IDig::new(1);

            let mut c = IDig::new(0);

            //buf.push(zero.clone()); //fuck

            let mut alpha = W.clone();

            let w = W.clone() * W.clone();



        }




}

fn works() {
    let mut W:usize = 0;

    let mut buf:Vec<usize> = Vec::new();

    loop {
        W += 1;

        let mut c: usize = 0;

        buf.push(0);

        let mut alpha = W.clone();

        let w = { W * W };

        for i in c .. W {

            let j = 0;

            let b = buf[i];

            c = c + b;

            for j in b .. alpha + 1 {
                if { i * i } + { j * j } < w {

                    c += 1;

                    alpha = j.clone()
                }


            }

            buf[i] = j;

        }


        println!("{}", ((c as f64) / (w as f64)) * 4.0);
    }


}

fn non_optimised() {

    let mut W:usize = 0;


    loop {
        W += 1;

        let mut c: usize = 0;


        let mut alpha = W.clone();

        let w = { W * W };

        for i in c .. W {


            for j in 0 .. alpha + 1 {
                if { i * i } + { j * j } < w {

                    c += 1;

                    alpha = j.clone()
                }


            }


        }


        println!("{} {}", c ,w );
    }

}

fn non_optimised_digger() {

    let mut W= IDig::new(0);


    loop {
        W = W.clone() + IDig::new(1);

        let mut c = IDig::new(0);


        let mut alpha = W.clone();

        let w = { W.clone() * W.clone() };

        for i in (W.clone() - c.clone()).init_iter()  {



            for j in (alpha.clone() + IDig::new(1)).init_iter() {


                if ({ i.clone() * i.clone() } + { j.clone() * j.clone() } )< w {

                    c = c + IDig::new(1);

                    alpha = j.clone()
                }


            }


        }


        println!("{} {} {}", c , (w), W );
    }


}
