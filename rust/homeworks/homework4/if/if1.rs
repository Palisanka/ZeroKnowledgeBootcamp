// if1.rs

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `zustlings hint if1` for hints
    // if (a > b) {
    //     a
    // } else {
    //     b
    // }
    // use a match expression

    // match a {
    //     a if a > b => {
    //         println!("a : {} b: {}",a, b);
    //         return a;
    //     },
    //     a if b > a =>  {
    //         println!("a : {} b: {}",a, b);
    //         return b;
    //     },
    //     _ => panic!(),
    // }

    match a.cmp(&b) {
        std::cmp::Ordering::Greater => a,
        std::cmp::Ordering::Less => b,
        std::cmp::Ordering::Equal => a,
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
