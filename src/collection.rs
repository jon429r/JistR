/// this file will store code for collections
/// there are 2 types, arrays and dictoinaries
/// arrays are unordered collections of variable amounts, can store one type of data declared at initialization
/// dictionaries are ordered collections of key value pairs, keys are one type, values are another type or the same
/// declared at runtime
/// arrays are declared with [a, b, c]
/// dictionaries are declared with {a=>1, b=>2, c=>3}

pub mod collections {
    use crate::base_variable::base_types::BaseTypes;
    //use crate::node::nodes::ASTNode;

    pub struct Array {
        pub name: String,
        pub data: Vec<BaseTypes>,
        pub value_type: BaseTypes,
    }

    // functions for arrays: new, push, pop, remove, get(i), set(i), to_string

    impl Array {
        pub fn new(name: String, value_type: BaseTypes, data: Vec<BaseTypes>) -> Array {
            Array {
                name,
                data,
                value_type,
            }
        }

        pub fn push(&mut self, value: BaseTypes) {
            self.data.push(value);
        }

        pub fn pop(&mut self) -> Option<BaseTypes> {
            self.data.pop()
        }

        pub fn append(&mut self, value: &mut Vec<BaseTypes>) {
            //add to end 
            self.data.append(value);
        }

        pub fn remove(&mut self, index: usize){
            if index < self.data.len() {
                Some(self.data.remove(index));
            } else {
                println!("Syntax Error, during removal")
            }
        }

        pub fn get(&self, index: usize) -> Option<BaseTypes> {
            return self.data.get(index).clone().cloned();
        }

        pub fn set(&mut self, index: usize, value: BaseTypes) -> Option<BaseTypes> {
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
        pub values: Vec<(BaseTypes, BaseTypes)>,
        pub types: (BaseTypes, BaseTypes),
    }

    // functions for dictionaries: new, add, remove, get, set(key), keys, values, to_string

    impl Dictionary {
        pub fn new(name: String, key_type: BaseTypes, value_type: BaseTypes, values: Vec<(BaseTypes, BaseTypes)>) -> Dictionary {
            Dictionary {
                name,
                values,
                types: (key_type, value_type),
            }
        }

        pub fn add(&mut self, key: BaseTypes, value: BaseTypes) {
            self.values.push((key, value));
        }

        /*pub fn append(&mut self, key: ASTNode, value: ASTNode) {
            // add to end of dict
            self.values.add((key, value));
        } */

        pub fn remove(&mut self, key: BaseTypes) -> Option<(BaseTypes, BaseTypes)> {
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

        pub fn get(&self, key: BaseTypes) -> Option<&(BaseTypes, BaseTypes)> {
            self.values.iter().find(|(k, _)| k == &key)
        }

        pub fn set(&mut self, key: BaseTypes, value: BaseTypes) -> Option<BaseTypes> {
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

        pub fn keys(&self) -> Vec<&BaseTypes> {
            self.values.iter().map(|(k, _)| k).collect()
        }

        pub fn values(&self) -> Vec<&BaseTypes> {
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
