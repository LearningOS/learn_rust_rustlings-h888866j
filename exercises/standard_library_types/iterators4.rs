// iterators4.rs



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    
    // let mut ori:Vec<u64> = (1..=num).collect();
    // let ret:u64 = 1;
    // let x = ori.iter().enumerate().map(|(i,v)| if i==0{v * ret}else{v * ori[i]} ).collect::<Vec<u64>>()[ori.len()-1];
    // x //不行

    // // 可以
    // let mut ret = 1;
    // for i in 1..=num{
    //     ret *= i
    // }
    // ret 
    
    // 可以
    // fn sss(n:u64) -> u64{
    //     if n >=1 { 
    //         return n * sss(n-1)
    //     }
    //     1
    //     // match n{
    //     //     1 => 1,
    //     //     _ if n>1 => return n * sss(n-1),
    //     //     _ => {0}
    //     // }
    // }
    // sss(num)
    
    // 可以，不仔细看文档都想不到
    (1..=num).product()

    // 不行
    // (1..=num)
    // .map(|x|
    //     match x {
    //         1 => {1},
    //         _ if x >1 => {
    //             x * (x-1)
    //         }
    //         _ => {0}
    //     }
    // ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
