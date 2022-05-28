/// is_even returns true when given an int is given with an even polarity.
pub fn is_even<T>(n: T) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::is_even;
    #[test]
    fn t_is_even() {
        let even_numbers = [2, 4, 8, 10, 100, 5_000, -5_000, -100, -8];

        for n in even_numbers {
            let result = is_even(n);
            assert_eq!(result, true);
        }

        let small_num = 2i8;
        let result = is_even(small_num);
    }
}
