mod optimizer {
    use ethabi::ParamType;
    use std::fs;

    pub fn run(function_name: &str, params: &[ParamType]) -> Option<String> {
        let suffixes = fs::read_to_string("db.txt").expect("Something went wrong reading the file");

        for suffix in suffixes.lines() {
            let new_function_name = format!("{}_{}", function_name, suffix);
            let encoded_new_function_signature =
                ethabi::short_signature(new_function_name.as_str(), params);

            if encoded_new_function_signature[0] == 0 && encoded_new_function_signature[1] == 0 {
                return Some(new_function_name);
            }
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_optimize_function_signature() {
            let function_name = "myFunction";
            let function_params = [ParamType::Address];

            let res = run(function_name, &function_params);
            assert_eq!(true, res.is_some())
        }
    }
}
