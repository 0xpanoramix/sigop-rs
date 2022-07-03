mod validator;

pub mod optimizer {
    use crate::validator::try_validate;
    use indicatif::ProgressBar;
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
        pub fn try_optimizations(&self, level: u8, target: u8, debug: bool) -> Option<String> {
            // Used by the accumulator to create combinations.
            let dictionary = vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
                "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "$", "_",
            ];
            // The vector which contains all combinations from previous round.
            // Used to accumulate combinations trough time.
            let mut acc: Vec<String> = dictionary.iter().map(|&c| c.to_string()).collect();
            // The vector which contains all combinations from current round.
            let mut cur: Vec<String> = vec![];

            // Debug.
            let dictionary_len = dictionary.len();
            let max_iterations: u64 = (0..=level)
                .into_iter()
                .map(|n| dictionary_len.pow(n as u32) as u64)
                .sum();
            let pb = ProgressBar::new(max_iterations);

            // Used to hold the function signature selector.
            let mut out = [0u8; 4];
            // Used by the string allocator later on.
            let func_sig_len = self.name.len() + self.args.len() + 1;
            // Used to avoid multiple dereferences.
            let name = &*self.name;
            let args = &*self.args;

            // For every suffix length until we reach level.
            for _n in 0..level {
                // For each element of the accumulator.
                for item in &acc {
                    if debug {
                        // Debug progress.
                        pb.inc(1);
                    }

                    // Declared here to avoid calling this multiple times in the next nested loop
                    // below.
                    let item_len = item.len();

                    // Now we iterate in each element of the dictionary.
                    for next in &dictionary {
                        // Used by the string allocator just below.
                        let combination_len = item_len + next.len();

                        // The combination contains the concatenation of item and next.
                        // We use with_capacity because it's faster compared to the format macro.
                        let mut combination = String::with_capacity(combination_len);
                        combination.push_str(item);
                        combination.push_str(next);

                        // It's faster compared to the format macro here too.
                        let mut func_sig = String::with_capacity(combination_len + func_sig_len);
                        func_sig.push_str(name);
                        func_sig.push('_');
                        func_sig.push_str(&*combination);
                        func_sig.push_str(args);
                        encode_function_signature(func_sig.as_str(), &mut out);

                        // Checks if the first elements are zeros.
                        let mut found = true;
                        for i in 0..target {
                            if out[i as usize] != 0 {
                                found = false;
                                break;
                            }
                        }
                        if found {
                            return Some(func_sig);
                        }

                        // We push the new combination in the current (temporary) vector.
                        cur.push(combination);
                    }
                }
                // Replaces the accumulator with new combinations from this round and clean-up the
                // temporary vector.
                acc = cur.to_owned();
                cur.clear();
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
    pub fn run(function_signature: &str, level: u8, target: u8, debug: bool) -> Option<String> {
        let function_signature_cleaned = try_validate(function_signature)?;
        let function = try_parse(function_signature_cleaned.as_str())?;

        function.try_optimizations(level, target, debug)
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
        fn it_tries_optimizations() {
            let f = Function {
                name: "myFunction".to_string(),
                args: "(address)".to_string(),
            };

            let optimization = f.try_optimizations(3, 2, false);
            assert_eq!(true, optimization.is_some());
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
