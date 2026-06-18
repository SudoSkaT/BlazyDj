pub fn err_kind() -> &'static str {
    "generic"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_err_kind() {
        assert_eq!(err_kind(), "generic");
    }
}
