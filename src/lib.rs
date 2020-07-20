// clp: simple command-line parser
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum ArgSpec {
    Never,    // 0
    Optional, // 0 or 1
    Required, // 1
}

#[derive(Debug, Copy, Clone)]
pub struct ArgType(pub ArgSpec, pub ArgSpec);

pub struct CLParser<'a> {
    args: &'a Vec<String>,
    arg_spec_map: HashMap<&'a str, ArgType>,
    arg_found_map: HashMap<&'a str, Option<&'a str>>,
    pub left_overs: Vec<&'a str>,
    flag_regex: Regex,
}

impl<'a> CLParser<'a> {
    pub fn new(args: &Vec<String>) -> CLParser {
        let arg_spec_map = HashMap::new();
        let arg_found_map = HashMap::new();
        let left_overs = vec![];
        let flag_regex = Regex::new(r"^--(\w+)\s*(\[)?(\w+)?(\])?$").unwrap();

        CLParser {
            args,
            arg_spec_map,
            arg_found_map,
            left_overs,
            flag_regex,
        }
    }

    fn trim_dashes(flag: &str) -> (&str, bool) {
        let dashes = flag.chars().take_while(|e| *e == '-').collect::<String>();
        (&flag[dashes.len()..], dashes.len() > 0)
    }

    fn get_arg(self_args: &Vec<String>, ix: usize) -> (Option<&str>, bool) {
        if ix < self_args.len() {
            let arg = &self_args[ix];
            let (flag, is_flag) = Self::trim_dashes(arg);
            (Some(flag), is_flag)
        } else {
            (None, false)
        }
    }

    pub fn define(&mut self, arg: &'a str) -> &mut Self {
        let err_msg = format!("Illegal flag specification: {}", arg);
        let cap = self.flag_regex.captures(arg);
        if let Some(cap) = cap {
            // 1:flag 2:[ 3:type 4:]
            let arg_spec = match (cap.get(2), cap.get(3), cap.get(4)) {
                (None, None, None) => ArgSpec::Never,             // "flag"
                (Some(_), Some(_), Some(_)) => ArgSpec::Optional, // "flag [ type ]"
                (None, Some(_), None) => ArgSpec::Required,       // "flag type"
                _ => panic!("{}", err_msg),
            };
            let flag = cap.get(1).unwrap().as_str();
            let arg_spec = ArgType(ArgSpec::Required, arg_spec);
            self.arg_spec_map.insert(flag, arg_spec);
            self
        } else {
            panic!("{}", err_msg)
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let mut ix = 1;

        while ix < self.args.len() {
            let (arg, is_flag) = Self::get_arg(&self.args, ix);
            let (next_arg, is_next_flag) = Self::get_arg(&self.args, ix + 1);
            let arg = arg.unwrap();

            if is_flag {
                // Have a flag ... check parameter
                let arg_spec = self.arg_spec_map.get(arg);
                if let Some(arg_spec) = arg_spec {
                    match arg_spec.1 {
                        ArgSpec::Never => {
                            if next_arg.is_some() && !is_next_flag {
                                return Err(format!(
                                    "Flag {:?} must not have a parameter, {:?} found.",
                                    arg,
                                    next_arg.unwrap()
                                ));
                            }
                        }
                        ArgSpec::Optional => {}
                        ArgSpec::Required => {
                            if is_next_flag || next_arg == None {
                                return Err(format!(
                                    "Flag {:?} needs to have a parameter, none found.",
                                    arg
                                ));
                            }
                        }
                    }
                } else {
                    return Err(format!("Invalid flag specified: {}.", arg));
                }
                // Ensure that we haven't already inserted this flag
                if self.arg_found_map.get(arg) != None {
                    return Err(format!("Duplicate flag found: {}", arg));
                }
                if is_next_flag {
                    self.arg_found_map.insert(arg, None);
                } else {
                    self.arg_found_map.insert(arg, next_arg);
                    ix += 1;
                }
            } else {
                // Not a flag ... bail out
                break;
            }
            ix += 1;
        }

        // Any remaining parameters shouldn't be flags
        while ix < self.args.len() {
            let (arg, is_flag) = Self::get_arg(&self.args, ix);
            let arg = arg.unwrap();
            if is_flag {
                let err = format!("Unexpected flag: {:?}.", arg);
                return Err(err);
            }
            self.left_overs.push(arg);
            ix += 1;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn get(&mut self, key: &str) -> Option<&str> {
        *self.arg_found_map.get(key).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split(args: &str) -> Vec<String> {
        args.split(' ').into_iter().map(|e| e.to_owned()).collect()
    }

    #[test]
    fn test_negative_bad_flag() {
        let args = tests::split("cmdname --hell --wuld");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello param").define("--world param");

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }

    #[test]
    fn test_negative_missing_param1() {
        let args = tests::split("cmdname --hello hello --world");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello param").define("--world param");

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }

    #[test]
    fn test_negative_missing_param2() {
        let args = tests::split("cmdname --hello --world");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello param").define("--world param");

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }

    #[test]
    fn test_negative_unwanted_param() {
        let args = tests::split("cmdname --hello world");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello").define("--world");

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }

    #[test]
    fn test_negative_repeated_flag() {
        let args = tests::split("cmdname --hello 1 --hello 2 --world 3");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello [param]").define("--world [param]");

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }

    #[test]
    fn test_all_positive() {
        let args = tests::split(
            "cmdname --hello 1hello --world wuldparam -how howparam --are -you youparam extra1 extra2",
        );
        let mut clpr = CLParser::new(&args);

        clpr.define("--hello [param]")
            .define("--world param")
            .define("--how [param]")
            .define("--are")
            .define("--you [param]");

        let retval = clpr.parse();
        clpr.get("hello");

        assert!(clpr.left_overs == vec!["extra1", "extra2"]);
        assert!(retval.is_ok());
        assert!(clpr.get("hello") == Some("1hello"));
        assert!(clpr.get("how") == Some("howparam"));
        assert!(clpr.get("are") == None);
    }

    #[test]
    fn test_positive1() {
        let args = tests::split("cmdname --hello");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello");

        let retval = clpr.parse();
        assert!(retval.is_ok());
    }
}
