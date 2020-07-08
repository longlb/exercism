pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    // Graph implementation
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, args: &[Node]) -> Self {
            self.nodes = args.to_vec();
            self
        }

        pub fn with_edges(mut self, args: &[Edge]) -> Self {
            self.edges = args.to_vec();
            self
        }

        pub fn with_attrs<'a>(mut self, args: &[(&'a str, &'a str)]) -> Self {
            for (key, value) in args {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn get_node(&self, arg: &str) -> Option<&Node> {
            self.nodes.iter().find(|x| x.name == arg)
        }
    }

    pub mod graph_items {
        // Node implmentation
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<'a>(mut self, args: &[(&'a str, &'a str)]) -> Self {
                    for (key, value) in args {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, arg: &str) -> Option<&str> {
                    match self.attrs.get(arg) {
                        Some(x) => Some(&x),
                        None => None,
                    }
                }
            }
        }

        // Edge implementation
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                p1: String,
                p2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(p1: &str, p2: &str) -> Self {
                    Edge {
                        p1: p1.to_string(),
                        p2: p2.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<'a>(mut self, args: &[(&'a str, &'a str)]) -> Self {
                    for (key, value) in args {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }
            }
        }
    }
}
