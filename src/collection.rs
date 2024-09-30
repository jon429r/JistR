/// this file will store code for collections
/// there are 2 types, arrays and dictoinaries
/// arrays are unordered collections of variable amounts, can store one type of data declared at initialization
/// dictionaries are ordered collections of key value pairs, keys are one type, values are another type or the same
/// declared at runtime
/// arrays are declared with [a, b, c]
/// dictionaries are declared with {a=>1, b=>2, c=>3}

pub mod collections {
    use crate::base_variable::base_types::BaseTypes;
    use crate::node::nodes::ASTNode;

    pub struct Array {
        pub name: String,
        pub data: Vec<ASTNode>,
        pub value_type: BaseTypes,
    }

    // functions for arrays: new, push, pop, remove, get(i), set(i), to_string

    impl Array {
        pub fn new(name: String, value_type: BaseTypes) -> Array {
            Array {
                name,
                data: Vec::new(),
                value_type,
            }
        }

        pub fn push(&mut self, value: ASTNode) {
            self.data.push(value);
        }

        pub fn pop(&mut self) -> Option<ASTNode> {
            self.data.pop()
        }

        pub fn remove(&mut self, index: usize) -> Option<ASTNode> {
            if index < self.data.len() {
                Some(self.data.remove(index))
            } else {
                None
            }
        }

        pub fn get(&self, index: usize) -> Option<&ASTNode> {
            self.data.get(index)
        }

        pub fn set(&mut self, index: usize, value: ASTNode) -> Option<ASTNode> {
            if index < self.data.len() {
                Some(std::mem::replace(&mut self.data[index], value))
            } else {
                None
            }
        }

        pub fn to_string(&self) -> String {
            let mut output = String::new();
            output.push_str(&self.name);
            output.push_str(" = [");
            for (i, value) in self.data.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&value.to_string());
            }
            output.push_str("]");
            output
        }
    }

    pub struct Dictionary {
        pub name: String,
        pub values: Vec<(ASTNode, ASTNode)>,
        pub types: (BaseTypes, BaseTypes),
    }

    // functions for dictionaries: new, add, remove, get, set(key), keys, values, to_string

    impl Dictionary {
        pub fn new(name: String, key_type: BaseTypes, value_type: BaseTypes) -> Dictionary {
            Dictionary {
                name,
                values: Vec::new(),
                types: (key_type, value_type),
            }
        }

        pub fn add(&mut self, key: ASTNode, value: ASTNode) {
            self.values.push((key, value));
        }

        pub fn remove(&mut self, key: ASTNode) -> Option<(ASTNode, ASTNode)> {
            let mut index = None;
            for (i, (k, _)) in self.values.iter().enumerate() {
                if k == &key {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                Some(self.values.remove(i))
            } else {
                None
            }
        }

        pub fn get(&self, key: ASTNode) -> Option<&(ASTNode, ASTNode)> {
            self.values.iter().find(|(k, _)| k == &key)
        }

        pub fn set(&mut self, key: ASTNode, value: ASTNode) -> Option<ASTNode> {
            let mut index = None;
            for (i, (k, _)) in self.values.iter().enumerate() {
                if k == &key {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                Some(std::mem::replace(&mut self.values[i].1, value))
            } else {
                None
            }
        }

        pub fn keys(&self) -> Vec<&ASTNode> {
            self.values.iter().map(|(k, _)| k).collect()
        }

        pub fn values(&self) -> Vec<&ASTNode> {
            self.values.iter().map(|(_, v)| v).collect()
        }

        pub fn to_string(&self) -> String {
            let mut output = String::new();
            output.push_str(&self.name);
            output.push_str(" = {");
            for (i, (key, value)) in self.values.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&key.to_string());
                output.push_str("=>");
                output.push_str(&value.to_string());
            }
            output.push_str("}");
            output
        }
    }
}
