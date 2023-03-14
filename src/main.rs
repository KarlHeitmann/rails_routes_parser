use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

pub struct RouteNode {
    domain: String,
    prefix: Option<String>,
    verb: String,
    uri_pattern: String,
    controller_action: String,
}

impl RouteNode {
    pub fn new(domain: String, s: String) -> Result<Self, &'static str> {
        let ss = s.split(" ");
        let ss = ss.filter(|s| !s.is_empty() );
        match ss.clone().count() {
            4 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: Some(ss.get(0).unwrap().to_string()), verb: ss.get(1).unwrap().to_string(), uri_pattern: ss.get(2).unwrap().to_string(), controller_action: ss.get(3).unwrap().to_string()
                })
            },
            3 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    domain,
                    prefix: None, verb: ss.get(0).unwrap().to_string(), uri_pattern: ss.get(1).unwrap().to_string(), controller_action: ss.get(2).unwrap().to_string()
                })
            }
            _ => {Err("Invalid number of strings in line")}
        }
    }
    pub fn route(&self, target: &String) -> Result<String, &'static str> {
        if target.starts_with("app/views/") {
            let (_, file) = target.split_at(10);
            let (file_name, extension) = file.split_once(".").unwrap();
            let mut file_data: Vec<&str> = file_name.split("/").collect();
            let action = file_data.pop().unwrap();
            // println!("{:?}||||{}", file_data.join("::"), action);
            let target = format!("{}#{}", file_data.join("/"), action);

            match self.controller_action.contains(&target) {
                true => Ok(format!("{}{} | controller_action: {} | target: {}", self.domain, self.uri_pattern, self.controller_action, target)),
                false => Err("controller_action don't contain target"),
            }
        } else {
            Err("Target doesn't starts with `app/views`")
        }
    }
}

pub struct Routes {
    // path: String,
    domain: String,
    route_nodes: Vec<RouteNode>,
}

impl Routes {
    pub fn new(domain: &str) -> Option<Self> {
        // Create a path to the desired file
        let path = Path::new("routes.txt");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let routes = match File::open(&path) {
            Err(_) => None,
            Ok(mut file) => { 
                let mut s = String::new();
                let res = match file.read_to_string(&mut s) {
                    Err(_) => None,
                    Ok(_) => {
                        // print!("{} contains:\n{}", display, s);
                        let mut route_nodes: Vec<RouteNode> = vec![];
                        let ss = s.split("\n");
                        for s in ss {
                            match RouteNode::new(domain.to_string(), s.to_string()) {
                                Ok(route_node) => route_nodes.push(route_node),
                                Err(_e) => {}
                            }
                        }
                        Some(Self {
                            domain: domain.to_string(),
                            route_nodes,
                        })
                    }
                };

                res
            }
        };

        routes
    }
    pub fn find(&self, target: String) -> Option<String> {
        let mut result = None;
        for route_node in &self.route_nodes {
            match route_node.route(&target) {
                Ok(route) => {
                    result = Some(route);
                    break;
                },
                Err(_) => {},
            }
        }
        result
    }
}


fn main() {
    let routes = Routes::new("http://localhost:3000").unwrap();
    println!("Hello, world!");
    for arg in env::args() {
        println!("{}", routes.find(arg.clone()).unwrap_or(format!("arg: {} not found in routes", arg)));
    }
    // let args: Vec<String> = env::args().collect();
    /*
    for arg in argv::iter() {
        // arg is a &'static OsStr.
        println!("{}", arg.to_string_lossy());
    }
    */
}
