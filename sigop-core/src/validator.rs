use log::error;

fn verify_parenthesis(function_sig: &str) -> bool {
    // The function signature must end with a closing parenthesis.
    if !function_sig.ends_with(')') {
        error!("last character must be a closing parenthesis");
        return false;
    }

    // There must be the same number of opening and closing parenthesis.
    if function_sig.matches('(').count() != function_sig.matches(')').count() {
        error!("number of opening parenthesis differs from number of closing parenthesis");
        return false;
    }

    true
}

/// Used to remove whitespaces from the given function signature.
fn remove_whitespaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn try_validate(function_sig: &str) -> Option<String> {
    let result = remove_whitespaces(function_sig);

    if !verify_parenthesis(result.as_str()) {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::validator::{remove_whitespaces, verify_parenthesis};

    #[test]
    fn it_removes_whitespaces() {
        let original = "myFunction ( address) ";
        let result = remove_whitespaces(original);

        assert_eq!("myFunction(address)", result)
    }

    #[test]
    fn it_verifies_ending_character() {
        let function_sig = "myFunction(address";
        let result = verify_parenthesis(function_sig);

        assert_eq!(false, result)
    }

    #[test]
    fn it_verifies_parenthesis_count() {
        let function_sig = "myFunction((address)";
        let result = verify_parenthesis(function_sig);

        assert_eq!(false, result)
    }

    #[test]
    fn it_validates_parenthesis_criteria() {
        let function_sig = "myFunction(address)";
        let result = verify_parenthesis(function_sig);

        assert_eq!(true, result)
    }
}
