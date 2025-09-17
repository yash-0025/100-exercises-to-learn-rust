/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
fn magic_number(n: u32) -> u32 {
    let even_number : u32 = 12;
    let odd_number : u32 = 13;
    let number : u32 = 17;

    if n % 2 == 0 {
       return even_number;
    } else if n % 3 == 0 {
        return odd_number;
    } else {
        return number;
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
