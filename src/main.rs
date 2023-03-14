use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct RouteNode {
    prefix: Option<String>,
    verb: String,
    uri_pattern: String,
    controller_action: String,
}

impl RouteNode {
    pub fn new(s: String) -> Result<Self, &'static str> {
        let ss = s.split(" ");
        let ss = ss.filter(|s| !s.is_empty() );
        match ss.clone().count() {
            4 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    prefix: Some(ss.get(0).unwrap().to_string()), verb: ss.get(1).unwrap().to_string(), uri_pattern: ss.get(2).unwrap().to_string(), controller_action: ss.get(3).unwrap().to_string()
                })
            },
            3 => {
                let ss: Vec<&str> = ss.collect();
                Ok(Self {
                    prefix: None, verb: ss.get(0).unwrap().to_string(), uri_pattern: ss.get(1).unwrap().to_string(), controller_action: ss.get(2).unwrap().to_string()
                })
            }
            _ => {Err("Invalid number of strings in line")}
        }
    }
}

pub struct Routes {
    route_nodes: Vec<RouteNode>,
}

impl Routes {
    pub fn new() -> Option<Self> {
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
                            match RouteNode::new(s.to_string()) {
                                Ok(route_node) => route_nodes.push(route_node),
                                Err(_e) => {}
                            }
                        }
                        Some(Self {
                            route_nodes,
                        })
                    }
                };

                res
            }
        };

        routes
    }
}


fn main() {
    Routes::new();
    println!("Hello, world!");
}
