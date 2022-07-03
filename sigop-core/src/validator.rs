/// Used to remove whitespaces from the given function signature.
fn remove_whitespaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn try_validate(function_sig: &str) -> Option<String> {
    let result = remove_whitespaces(function_sig);

    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::validator::remove_whitespaces;

    #[test]
    fn it_removes_whitespaces() {
        let original = "myFunction ( address) ";
        let result = remove_whitespaces(original);

        assert_eq!("myFunction(address)", result)
    }
}
