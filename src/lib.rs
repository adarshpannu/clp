
use std::collections::HashMap;


pub struct CLParser<'a> {
    pub args: &'a Vec<String>,
    pub arg_map: HashMap<&'a String, Option<&'a String>>,
    pub left_overs: Vec<&'a String>
}

impl<'a> CLParser<'a> {
    pub fn new(args: &'a Vec<String>) -> CLParser {
        let mut arg_map = HashMap::new();
        let mut flag_opt: Option<&String> = None;
        let mut left_overs: Vec<&String> = vec![];

        for arg in args.iter() {
            if arg.find('-').is_some() {
                // Saw a flag (--flag)
                if let Some(flag) = flag_opt {
                    // Previous flag didn't have a parameter.
                    arg_map.insert(flag, None);
                }
                flag_opt = Some(arg);
            } else {
                // Saw a parameter or leftover
                if let Some(flag) = flag_opt {
                    arg_map.insert(flag, Some(arg));
                    flag_opt = None;
                } else {
                    left_overs.push(arg);
                }
            }
        }
        CLParser { args, arg_map, left_overs }
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

        let args = vec!["--hello", "1", "--world", "-how", "--are", "1,4,5", "-you"];
        let args = args.iter().map(|&e| e.to_owned()).collect::<Vec<String>>();
        let mut hello: Option<String> = None;
        let mut world: Option<String> = None;
        let mut how: Option<bool> = None;

        let clp = super::CLParser::new(&args);

        clp.parse_string("hello", &mut hello)
            .parse_string("world", &mut world)
            .parse_bool("how", &mut how);

        println!("arg hash map: {:?}", clp.arg_map);
    }
}
