#[cfg(feature = "a")]
include!("a.rs");

#[cfg(not(feature = "a"))]
include!("b.rs");

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "a")]
    fn test_a() {
        let result = demo!();
        assert_eq!(result, "A");
    }

    #[test]
    #[cfg(not(feature = "a"))]
    fn test_a() {
        let result = demo!();
        assert_eq!(result, "B");
    }
}
