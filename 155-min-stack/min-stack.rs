struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }
    
    fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        if self.min_stack.last() == self.stack.last() {
            self.min_stack.pop();
        }
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        match self.stack.last() {
            Some(top) => *top,
            None => -1
        }
    }
    
    fn get_min(&self) -> i32 {
        match self.min_stack.last() {
            Some(min) => *min,
            None => -1
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */