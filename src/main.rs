fn fib(num: u32) -> u32 {
    if num < 2 {
        return num;
    }
    let mut f1: u32 = 0;
    let mut f2: u32 = 1;
    let mut fib: u32;
    let mut count = 1;
    loop {
        fib = f1 + f2;
        f1 = f2;
        f2 = fib;
        count+=1;
        if count == num{
            return fib;
        }
    }
}


fn main() {

    for num in 0..100 {
        println!("{} - {}", num, fib(num));
    }

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero(){
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn one(){
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn ten(){
        assert_eq!(fib(10), 55);
    }

    #[test]
    #[should_panic]
    fn overflow(){
        fib(48);
    }

}
