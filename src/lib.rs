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
    pub fn new(args: &'a Vec<String>) -> CLParser {
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

    pub fn parse(&mut self) {
        let mut left_overs: Vec<&String> = vec![];
        let mut ix = 0;

        println!("arg_spec_map {:?}", self.arg_spec_map);

        while ix < self.args.len() {
            let (arg, is_flag) = self.get_arg(ix);
            let (next_arg, is_next_flag) = self.get_arg(ix + 1);

            println!("arg = {:?}, next_arg = {:?}", arg, next_arg);
            if is_flag {
                // Have a flag ... check parameter
                let arg_spec = self.arg_spec_map.get(arg.unwrap()).unwrap();
                match arg_spec.1 {
                    ArgSpec::Never => {
                        if !is_next_flag {
                            panic!("Flag {:?} must not have a parameter, {:?} found.", arg, next_arg);
                        }
                    }
                    ArgSpec::Optional => {
                    }
                    ArgSpec::Required => {
                        if is_next_flag {
                            panic!("Flag {:?} needs to have a parameter, none found.", arg);
                        }
                    }
                }
            }
            ix += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArgSpec::*;
    use super::*;

    #[test]
    fn it_works() {
        println!("------------------");
        let args = vec![
            "--hello",
            "1",
            "extra_param",
            "--world",
            "wuld param",
            "-how",
            "--are",
            "-you",
            "another_extra_param",
        ];
        let args = args.iter().map(|&e| e.to_owned()).collect::<Vec<String>>();

        let mut clpr = CLParser::new(&args);

        clpr.define("--hello", ArgType(Required, Optional))
            .define("--world", ArgType(Required, Required))
            .define("--how", ArgType(Required, Optional))
            .define("--are", ArgType(Required, Never))
            .define("--you", ArgType(Required, Optional))
            .parse();

        println!("arg hash map: {:?}", clpr.arg_spec_map);
    }
}
