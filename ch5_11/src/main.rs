#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Q7(i8); //example of tuple struct
// conversion using the From should be mathematically equivalent.
// for type conversions that can fail, use std::convert::TryFrom
impl From<f64> for Q7 {
    fn from (n: f64) -> Self {
        // assert! (n >= -1.0); option to handle unexpected data
        // asser! (n< = 1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8) // truncate out-of-bounds input
        }
    }
}
// introduces impl From<T> for U
// its is a trait from std::convert::From
// it requires type U to implement from() that takes T as its argument
impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        // its safe to use f64 due to a number that can be represented 
        // in f32 can be represented in f64
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        // converting f64 into a f32 risks a loss of precision.
        // in this example it is safe due to i'm using only -1 and 1
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests { //defines a submodule in this file
    // brings the parent module within submodule's local scope.
    // items that are marked as pub can be accessed here
    use super::*;
    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.0), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

}
fn main() {
    println!("Hello, world!");
}
