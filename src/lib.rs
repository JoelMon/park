use num_integer::Integer;

/// Returns true when given an integer with an even polarity.
///
/// # Examples
///
/// ```rust
/// use park;
///
/// let result = park::is_even(2);
/// assert_eq!(result, true);
///
/// let result = park::is_even(3);
/// assert_eq!(result, false);
/// ```
pub fn is_even<N: Integer + Copy>(n: N) -> bool {
    let _0 = N::zero();
    let _1 = N::one();
    let _2 = _1 + _1;

    if n % _2 == _0 {
        return true;
    } else {
        return false;
    }
}

/// Returns true when given an integer with an odd polarity.
///
/// # Examples
///
/// ```rust
/// use park;
///
/// let result = park::is_odd(3);
/// assert_eq!(result, true);
///
/// let result = park::is_odd(2);
/// assert_eq!(result, false);
/// ```
pub fn is_odd<N: Integer + Copy>(n: N) -> bool {
    !is_even(n)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn t_is_even() {
        let even_numbers = [0, 2, 4, 8, 10, 100, 5_000, -5_000, -100, -8];
        let odd_numbers = [3, 5, 7, 11, 101, 5_033, -5_033, -103, -9];

        for n in even_numbers {
            let result = is_even(n);
            assert_eq!(result, true);
        }

        for n in odd_numbers {
            let result = is_even(n);
            assert_eq!(result, false);
        }

        let i16_num = [0i16, 2i16, 4i16];

        for n in i16_num {
            let result = is_even(n);
            assert_eq!(result, true);
        }

        let i8_num = [0i8, 2i8, 4i8];

        for n in i8_num {
            let result = is_even(n);
            assert_eq!(result, true);
        }

        let u8_num = [0u8, 2u8, 4u8];

        for n in u8_num {
            let result = is_even(n);
            assert_eq!(result, true);
        }

        let usize_num = [0usize, 2usize, 4usize];

        for n in usize_num {
            let result = is_even(n);
            assert_eq!(result, true);
        }
    }
    #[test]
    fn t_is_odd() {
        let odd_num = [3, 7, 11, 33, 45, -63, -777];
        let even_num = [2, 4, 14, 30, 44, -62, -770];

        for n in odd_num {
            let result = is_odd(n);
            assert_eq!(result, true);
        }

        for n in even_num {
            let result = is_odd(n);
            assert_eq!(result, false);
        }
    }
}
