pub fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("Zero is not a valid argument"),
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
     #[test]
    fn fib_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn fib_2() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn fib_3() {
        assert_eq!(fib(3), 2);
    }
}
