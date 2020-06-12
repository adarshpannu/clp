#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgSpec {
    Never,    // 0
    Optional, // 0 or 1
    Required, // 1
}

#[derive(Debug)]
pub struct ArgType(ArgSpec, ArgSpec);

pub struct CLParser<'a> {
    pub args: &'a Vec<String>,
    pub arg_spec_map: HashMap<&'a str, ArgType>,
    pub arg_found_map: HashMap<&'a str, &'a str>,
}

impl<'a> CLParser<'a> {
    pub fn new(args: &Vec<String>) -> CLParser {
        let arg_spec_map = HashMap::new();
        let arg_found_map = HashMap::new();
        CLParser {
            args,
            arg_spec_map,
            arg_found_map,
        }
    }

    fn trim_dashes(flag: &str) -> (&str, bool) {
        if flag.find("-") == Some(0) {
            if flag.find("--") == Some(0) {
                (&flag[2..], true)
            } else {
                (&flag[1..], true)
            }
        } else {
            (flag, false)
        }
    }

    fn get_arg(&self, ix: usize) -> (Option<&str>, bool) {
        if ix < self.args.len() {
            let arg = &self.args[ix];
            let (flag, is_flag) = Self::trim_dashes(arg);
            (Some(flag), is_flag)
        } else {
            (None, false)
        }
    }

    pub fn define(&mut self, arg: &'a str, arg_type: ArgType) -> &mut Self {
        let (flag, is_flag) = Self::trim_dashes(arg);
        if is_flag {
            self.arg_spec_map.insert(flag, arg_type);
        } else {
            panic!("Illegal flag specification: {}", arg);
        }
        self
    }

    pub fn parse(&'a mut self) -> Result<(), String> {
        let mut left_overs: Vec<&String> = vec![];
        let mut ix = 0;
        let mut arg_found_map = HashMap::new();

        while ix < self.args.len() {
            let (arg, is_flag) = self.get_arg(ix);
            let (next_arg, is_next_flag) = self.get_arg(ix + 1);

            if is_flag {
                // Have a flag ... check parameter
                let arg = arg.unwrap();

                let arg_spec = self.arg_spec_map.get(arg);
                if arg_spec.is_none() {
                    let err = format!("Invalid flag specified command line: {}.", arg);
                    return Err(err);
                }
                let arg_spec = arg_spec.unwrap();
                match arg_spec.1 {
                    ArgSpec::Never => {
                        if !is_next_flag {
                            let err = format!(
                                "Flag {:?} must not have a parameter, {:?} found.",
                                arg, next_arg
                            );
                            return Err(err);
                        }
                    }
                    ArgSpec::Optional => {}
                    ArgSpec::Required => {
                        if is_next_flag || next_arg == None {
                            let err =
                                format!("Flag {:?} needs to have a parameter, none found.", arg);
                            return Err(err);
                        }
                    }
                }
                if next_arg != None {
                    arg_found_map.insert(arg, next_arg.unwrap());
                }
            }
            ix += 1;
        }
        //self.arg_found_map = arg_found_map;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::ArgSpec::*;
    use super::*;

    #[test]
    fn test_all_positive() {
        println!("------------------");

        let args = tests::split(
            "--hello 1 extra_param --world wuldparam -how --are -you another_extra_param",
        );
        let mut clpr = CLParser::new(&args);

        clpr.define("--hello", ArgType(Required, Optional))
            .define("--world", ArgType(Required, Required))
            .define("--how", ArgType(Required, Optional))
            .define("--are", ArgType(Required, Never))
            .define("--you", ArgType(Required, Optional));

        println!("arg_found_map = {:?}", clpr.arg_found_map);
        assert!(clpr.parse().is_ok());

    }

    fn split(args: &str) -> Vec<String> {
        args.split(' ').into_iter().map(|e| e.to_owned()).collect()
    }

    #[test]
    fn test_positive1() {
        let args = tests::split("cmdname --hello");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello", ArgType(Required, Optional));

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_ok());
    }

    #[test]
    fn test_negative1() {
        let args = tests::split("cmdname --hello --world");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello", ArgType(Optional, Required))
            .define("--world", ArgType(Optional, Optional));

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }

    #[test]
    fn test_negative2() {
        let args = tests::split("cmdname --hello world");
        let mut clpr = CLParser::new(&args);
        clpr.define("--hello", ArgType(Optional, Never))
            .define("--world", ArgType(Optional, Never));

        let retval = clpr.parse();
        println!("retval: {:?}", retval);
        assert!(retval.is_err());
    }


    #[test]
    fn tokenize() {
        let mut str = "[--hello [int]]".to_owned();

        let replacements = ["[", "]", "-", "|"];
        for &fromstr in replacements.iter() {
            let tostr = format!(" {} ", fromstr);
            str = str.replace(fromstr, &tostr);
        }

        let tokens = str.split(' ').map(|e| e.trim()).filter(|&e| e.len() > 0);

        let mut stack: Vec<&str> = vec![];
        for token in tokens {
            match token {
                " " => {}
                "-" => {
                    if stack.last() == Some(&"-") {
                    } else {
                        stack.push(token)
                    }
                }
                _ => stack.push(token),
            }
        }

        for token in stack {
            print!("{} ", token);
        }
    }
}
