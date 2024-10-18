// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {
    // pub fn d0_factorial( acc:u64 ,nun :u64)->u64{
    //     if nun==0||nun==1{
    //          acc
    //     }else{
            
    //         d0_factorial(acc*nun, nun-1)
    //     }
    // }
    // d0_factorial(1, num)

    (0..=num).fold(1, |acc,num: u64|{
        match num{
            0|1=> acc,
            _=>acc*num,
        }
    })


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
