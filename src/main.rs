#![allow(non_snake_case)]

mod meth_with_more_digits;


fn main() {


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


        println!("{}", ((c as f64) / (W as f64 * W as f64)) * 4.0);
    }



}
