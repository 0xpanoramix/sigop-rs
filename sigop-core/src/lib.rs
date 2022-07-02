pub mod optimizer {
    use regex::Regex;
    use sha3::{Digest, Keccak256};

    #[derive(Default)]
    struct Function {
        name: String,
        args: String,
    }

    impl Function {
        /// This function generates combinations based on a dictionary and generates a function
        /// signature for each one of them.
        /// If the function signature contains at least the target number of zeros at the beginning,
        /// the optimization is found.
        pub fn try_optimisations(&self, level: u8, target: u8) -> Option<String> {
            let dictionary = vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
                "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
            ];
            let mut acc: Vec<String> = dictionary.iter().map(|&c| c.to_string()).collect();
            let mut out = [0u8; 4];

            for _n in 0..level {
                // Trying optimisations using combinations.
                for item in &acc {
                    encode_function_signature(
                        format!("{}_{}{}", self.name, item, self.args).as_str(),
                        &mut out,
                    );

                    let mut found = true;
                    for i in 0..target {
                        if out[i as usize] != 0 {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        return Some(format!("{}_{}{}", self.name, item, self.args));
                    }
                }

                acc = acc
                    .into_iter()
                    .flat_map(|c| dictionary.iter().map(move |&d| d.to_owned() + &*c))
                    .collect();
            }
            None
        }
    }

    /// This function computes the selector for a given function signature.
    fn encode_function_signature(function_signature: &str, out: &mut [u8]) {
        let data: Vec<u8> = From::from(function_signature);

        out.copy_from_slice(&Keccak256::digest(&data)[..out.len()])
    }

    // Tries to parse the function signature and splits the name from the arguments.
    fn try_parse(function_signature: &str) -> Option<Function> {
        let re = Regex::new(r"(.*?)\((.*)").unwrap();
        let caps = re.captures(function_signature)?;

        match caps.len() == 3 {
            true => Some(Function {
                name: caps.get(1).unwrap().as_str().to_string(),
                args: format!("({}", caps.get(2).unwrap().as_str()),
            }),
            false => None,
        }
    }

    /// Runs the optimizer on the given function signature.
    /// The level and target are used to indicate the optimizer when it should stop.
    pub fn run(function_signature: &str, level: u8, target: u8) -> Option<String> {
        let function = try_parse(function_signature)?;

        function.try_optimisations(level, target)
    }

    #[cfg(test)]
    mod tests {
        use crate::optimizer::{encode_function_signature, try_parse, Function};

        #[test]
        fn it_encodes_function_signature() {
            let signature = "myFunction(address)";
            let mut result = [0u8; 4];

            encode_function_signature(signature, &mut result);
            assert_eq!("a207136c", hex::encode(result))
        }

        #[test]
        fn it_tries_optimisations() {
            let f = Function {
                name: "myFunction".to_string(),
                args: "(address)".to_string(),
            };

            let optimisation = f.try_optimisations(3, 2);
            assert_eq!(true, optimisation.is_some());
        }

        #[test]
        fn it_parses_valid_function_signature() {
            let signature = "myFunction(address)";
            let function = try_parse(signature);

            assert_eq!(true, function.is_some());
            assert_eq!("myFunction", function.as_ref().unwrap().name);
            assert_eq!("(address)", function.as_ref().unwrap().args)
        }

        #[test]
        fn it_parses_valid_function_signature_with_struct() {
            let signature = "myFunction((address,uint256))";
            let function = try_parse(signature);

            assert_eq!(true, function.is_some());
            assert_eq!("myFunction", function.as_ref().unwrap().name);
            assert_eq!("((address,uint256))", function.as_ref().unwrap().args)
        }
    }
}
