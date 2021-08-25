#![allow(dead_code)]//XXX
use std::mem;

use super::Context;

#[derive(Debug)]
pub struct ContextStack {
    stack: Vec<Context>,
    current: Context,
}

impl ContextStack {
    pub fn new(root: Context) -> Self {
        Self {
            stack: Vec::new(),
            current: root,
        }
    }
    fn begin_context(&mut self, ctx: Context) {
        self.stack.push(mem::replace(&mut self.current, ctx));
    }
    // fn pop_context
}
