use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>)
}
impl Solution {
    pub fn deserialize(mut s: String) -> NestedInteger {
        let mut context = vec![];
        let mut temp = String::new();
        for c in s.chars() {
            match c {
                '-' | '0'..='9' => {
                    temp.push(c);
                },
                _ => {
                    if !temp.is_empty() {
                        let num = temp.parse::<i32>().unwrap();
                        temp.clear();
                        if context.is_empty() {
                            return NestedInteger::Int(num);
                        } else {
                            if let Some(NestedInteger::List(list)) = context.last_mut() {
                                list.push(NestedInteger::Int(num));
                            }
                        }   
                    }
                    match c {
                        '[' => {
                            context.push(NestedInteger::List(vec![]));
                        },
                        ']' => {
                            if context.len() == 1 {
                                return context.pop().unwrap();
                            } else {
                                let item = context.pop().unwrap();
                                if let Some(NestedInteger::List(list)) = context.last_mut() {
                                    list.push(item);
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        NestedInteger::Int(temp.parse::<i32>().unwrap())
    }
}