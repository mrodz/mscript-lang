use std::collections::HashMap;
use std::fmt::{Debug, Display};

use super::variable::Primitive;

type VariableMapping = HashMap<String, Primitive>;

#[derive(Debug)]
struct StackFrame {
    label: String,
    variables: VariableMapping,
}

#[derive(Debug)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn get_frame_label(&self) -> &String {
        &self.0.last().expect("nothing in the stack").label
    }

    pub fn get_frame_variables(&self) -> &VariableMapping {
        &self.0.last().expect("nothing in the stack").variables
    }

    pub fn extend(&mut self, label: &String) {
        self.0.push(StackFrame {
            label: label.clone(),
            variables: HashMap::new(),
        });
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn find_name(&self, name: &String) -> Option<&Primitive> {
        for i in (0..self.size() - 1).rev() {
            if let Some(var) = self.0[i].variables.get(name) {
                return Some(var);
            }
        }

        None
    }

    pub fn register_variable(&mut self, name: String, var: Primitive) {
        self.0
            .last_mut()
            .expect("nothing in the stack")
            .variables
            .insert(name, var);
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(first) = self.0.last() else {
            return write!(f, "<Empty Stack>")
        };

        write!(f, "\t>> {}", first.label)?; // print the cause first

        for stack_frame in self.0[..self.size() - 1].iter().rev() {
            write!(f, "\r\n\t ^ {}", stack_frame.label)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn add_frame() {
        // let mut stack = ::new("Top".into());
        let mut stack = Stack::new();

        let one = String::from("Main");
        let two = String::from("hi");

        stack.extend(&one);
        stack.extend(&two);

        // stack.extend("hi".into()).extend("bye".into());

        // dbg!(stack);
    }
}
