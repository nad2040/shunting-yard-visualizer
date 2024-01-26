use crate::token::{Assoc, Token, TokenValue};
use std::collections::VecDeque;

pub struct ShuntingYardParser {
    input_queue: VecDeque<Token>,
    operator_stack: VecDeque<Token>,
    pub output_queue: VecDeque<Token>,
}

impl ShuntingYardParser {
    pub fn new(input_queue: Vec<Token>) -> Self {
        let mut syp = Self {
            input_queue: VecDeque::from(input_queue),
            operator_stack: VecDeque::new(),
            output_queue: VecDeque::new(),
        };
        syp.parse();
        syp
    }

    pub fn emit(&self) {
        for t in &self.output_queue {
            println!("{:?}", t);
        }
    }

    fn parse(&mut self) {
        let known_functions = vec![String::from("max"), String::from("sin")];
        let known_bindings = vec![String::from("x"), String::from("y")];

        while let Some(t) = self.input_queue.pop_front() {
            // self.debug_stack();
            match &t.value {
                TokenValue::Integer(_) | TokenValue::Float(_) => self.output_queue.push_back(t),
                _variable if t.value.is_binding(&known_bindings) => self.output_queue.push_back(t),
                _function if t.value.is_func(&known_functions) => self.operator_stack.push_front(t),
                _operator if t.value.is_op() => {
                    while let Some(next_t) = self.operator_stack.front() {
                        if next_t.value.is_op()
                            && next_t.value != TokenValue::LeftParen
                            && (next_t.value.precedence() > t.value.precedence()
                                || (t.value.precedence() == next_t.value.precedence()
                                    && t.value.assoc().unwrap() == Assoc::Left))
                        {
                            let Some(next_t) = self.operator_stack.pop_front() else { todo!() };
                            self.output_queue.push_back(next_t);
                        } else {
                            break;
                        }
                    }
                    self.operator_stack.push_front(t);
                }
                TokenValue::Comma => {
                    while let Some(next_t) = self.operator_stack.front() {
                        if next_t.value.is_op() && next_t.value != TokenValue::LeftParen {
                            let Some(next_t) = self.operator_stack.pop_front() else { todo!() };
                            self.output_queue.push_back(next_t);
                        } else {
                            break;
                        }
                    }
                }
                TokenValue::LeftParen => {
                    self.operator_stack.push_front(t);
                }
                TokenValue::RightParen => {
                    while let Some(next_t) = self.operator_stack.front() {
                        if next_t.value.is_op() && next_t.value != TokenValue::LeftParen {
                            let Some(next_t) = self.operator_stack.pop_front() else { todo!() };
                            self.output_queue.push_back(next_t);
                        } else {
                            break;
                        }
                    }
                    if self.operator_stack.is_empty() {
                        panic!("Mismatched parentheses!");
                    }
                    assert!(self
                        .operator_stack
                        .front()
                        .is_some_and(|t| { t.value == TokenValue::LeftParen }));
                    self.operator_stack.pop_front(); // pop '('
                    if let Some(next_t) = self.operator_stack.front() {
                        if next_t.value.is_func(&known_functions) {
                            let Some(next_t) = self.operator_stack.pop_front() else { todo!() };
                            self.output_queue.push_back(next_t);
                        }
                    }
                }
                _ => todo!(),
            }
        }
        while let Some(t) = self.operator_stack.pop_front() {
            match t.value {
                TokenValue::LeftParen => {
                    panic!("Mismatched parentheses!");
                }
                _ => {
                    self.output_queue.push_back(t);
                }
            }
        }
    }

    fn debug_stack(&self) {
        print!("[");
        for op_func in &self.operator_stack {
            print!("{:?}, ", op_func.value);
        }
        println!("]");
    }
}
