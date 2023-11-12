pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn fibonacci_iter(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;
    for _ in 1..n {
        res = a + b;
        a = b;
        b = res;
    }
    res
}

//return result and previous number
pub fn fibonacci_dynamic(n: u32) -> (u32, u32) {
    if n <= 1 {
        return (1, 1);
    }

    let (res, prev) = fibonacci_dynamic(n - 1);
    (prev + res, res)
}

pub fn fibonacci_tail_recursive(n: u32, curr: u32, prev_sum: u32, curr_sum: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    if n == curr {
        return curr_sum;
    }

    return fibonacci_tail_recursive(n, curr + 1, curr_sum, curr_sum + prev_sum);
}

fn main() {
    for i in 0..10 {
        println!(
            "naive: {}, iter: {}, dyn: {}, tail: {}",
            fibonacci(i),
            fibonacci_iter(i),
            fibonacci_dynamic(i).0,
            fibonacci_tail_recursive(i, 1, 1, 1)
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonaccis() {
        assert_eq!(fibonacci(14), fibonacci_iter(14));
        assert_eq!(
            fibonacci_tail_recursive(10, 1, 1, 1),
            fibonacci_dynamic(10).0
        );
        assert_eq!(fibonacci_tail_recursive(12, 1, 1, 1), fibonacci_iter(12));
    }
}
