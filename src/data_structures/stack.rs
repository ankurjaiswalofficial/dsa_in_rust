//! Stack implementation backed by `Vec<T>`.
//!
//! A stack is last-in, first-out (LIFO). It is useful for parsing, DFS, and undo systems.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::stack::{is_balanced_parentheses, Stack};
//!
//! let mut stack = Stack::new();
//! stack.push(10);
//! stack.push(20);
//! assert_eq!(stack.pop(), Some(20));
//! assert_eq!(stack.peek(), Some(&10));
//! assert!(is_balanced_parentheses("([]{})"));
//! ```

#[derive(Debug, Clone, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MinStack<T> {
    values: Vec<T>,
    mins: Vec<T>,
}

impl<T: Ord + Clone> MinStack<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            mins: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        if self.mins.last().is_none_or(|current| &value <= current) {
            self.mins.push(value.clone());
        }
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.values.pop()?;
        if self.mins.last().is_some_and(|min| *min == value) {
            self.mins.pop();
        }
        Some(value)
    }

    pub fn min(&self) -> Option<&T> {
        self.mins.last()
    }
}

pub fn is_balanced_parentheses(text: &str) -> bool {
    let mut stack = Vec::new();
    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

pub fn infix_to_postfix(expression: &str) -> String {
    fn precedence(op: char) -> i32 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            '^' => 3,
            _ => 0,
        }
    }

    let mut output = String::new();
    let mut operators = Vec::new();

    for ch in expression.chars().filter(|ch| !ch.is_whitespace()) {
        if ch.is_alphanumeric() {
            output.push(ch);
        } else if ch == '(' {
            operators.push(ch);
        } else if ch == ')' {
            while let Some(top) = operators.pop() {
                if top == '(' {
                    break;
                }
                output.push(top);
            }
        } else {
            while let Some(&top) = operators.last() {
                if top == '(' || precedence(top) < precedence(ch) {
                    break;
                }
                output.push(operators.pop().expect("operator exists"));
            }
            operators.push(ch);
        }
    }

    while let Some(op) = operators.pop() {
        if op != '(' {
            output.push(op);
        }
    }
    output
}

pub fn infix_to_prefix(expression: &str) -> String {
    let reversed: String = expression
        .chars()
        .rev()
        .map(|ch| match ch {
            '(' => ')',
            ')' => '(',
            other => other,
        })
        .collect();
    infix_to_postfix(&reversed).chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::{MinStack, Stack, infix_to_postfix, infix_to_prefix, is_balanced_parentheses};

    #[test]
    fn stack_is_lifo() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_applications_are_covered() {
        assert!(is_balanced_parentheses("([]{})"));
        assert!(!is_balanced_parentheses("(]"));
        assert_eq!(infix_to_postfix("a+b*c"), "abc*+");
        assert_eq!(infix_to_prefix("a+b*c"), "+a*bc");

        let mut min_stack = MinStack::new();
        min_stack.push(3);
        min_stack.push(5);
        min_stack.push(2);
        assert_eq!(min_stack.min(), Some(&2));
        min_stack.pop();
        assert_eq!(min_stack.min(), Some(&3));
    }
}
