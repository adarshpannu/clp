
use std::collections::HashMap;


pub struct CLParser<'a> {
    pub args: &'a Vec<String>,
    pub arg_map: HashMap<&'a str, Option<&'a String>>,
    pub left_overs: Vec<&'a String>
}

impl<'a> CLParser<'a> {
    pub fn new(args: &'a Vec<String>) -> CLParser {
        let mut arg_map = HashMap::new();
        let mut flag_opt: Option<&str> = None;
        let mut prev_flag_opt: Option<&str> = None;

        let mut left_overs: Vec<&String> = vec![];

        for arg in args.iter() {

            flag_opt = Self::parse_flag(arg);
            if let Some(flag_str) = flag_opt {
                // Saw a flag (--flag)
                arg_map.insert(flag_str, None);
                prev_flag_opt = flag_opt;
            } else {
                // Saw a parameter or leftover
                if let Some(prev_flag) = prev_flag_opt {
                    arg_map.insert(prev_flag, Some(arg));
                } else {
                    left_overs.push(arg);
                }
                prev_flag_opt = None;
            }
        }
        CLParser { args, arg_map, left_overs }
    }

    fn parse_flag(flag: &String) -> Option<&str> {
        if flag.find("-") == Some(0) {
            if flag.find("--") == Some(0) {
                Some(&flag[2..])
            } else {
                Some(&flag[1..])
            }
        } else {
            None
        }
    }

    pub fn parse_string(&self, _name: &str, retval: &mut Option<String>) -> &Self {
        *retval = None;
        self
    }

    pub fn parse_bool(&self, _name: &str, retval: &mut Option<bool>) -> &Self {
        *retval = None;
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("Hello");

        let args = vec!["--hello", "1", "extra_param", "--world", "-how", "--are", "1,4,5", "-you", "another_extra_param"];
        let args = args.iter().map(|&e| e.to_owned()).collect::<Vec<String>>();
        let mut hello: Option<String> = None;
        let mut world: Option<String> = None;
        let mut how: Option<bool> = None;

        let clp = super::CLParser::new(&args);

        clp.parse_string("hello", &mut hello)
            .parse_string("world", &mut world)
            .parse_bool("how", &mut how);

        println!("arg hash map: {:?}", clp.arg_map);
        println!("left_overs: {:?}", clp.left_overs);

    }
}
