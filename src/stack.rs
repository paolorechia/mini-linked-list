use super::linked_list::LinkedList;

pub struct Stack<T> {
    pub length: usize,
    items: LinkedList<T>
}

pub struct StackPopResult<T> {
    pub stack: Stack<T>,
    pub value: Option<T>
}

impl Stack<i32> {
    pub fn new() -> Stack<i32> {
        Stack {
            items: LinkedList::new(),
            length: 0
        }
    }
    pub fn push(mut self, x: i32) -> Stack<i32>{
        let list = self.items.push_left(x);
        self.items = list;
        self.length += 1;
        return self
    }

    pub fn pop(mut self) -> StackPopResult<i32> {
        let result = self.items.pop_left();
        match result.list {
            Some(list) => {
                self.items = *list;
            }
            None => {
                self.items = LinkedList::<i32>::new();
            }
        };
        StackPopResult {
            stack: self,
            value: result.val
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_new() {
        Stack::new();
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack = stack.push(1);
        stack = stack.push(2);
        assert_eq!(stack.items.collect(), vec![2, 1]);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.items.push_right(1);
        stack.items.push_right(2);

        let result = stack.pop();
        assert_eq!(result.value.unwrap(), 1);
        let stack = result.stack;

        let result = stack.pop();
        assert_eq!(result.value.unwrap(), 2);
        let stack = result.stack;

        let result = stack.pop();
        assert_eq!(result.value.is_none(), true);
        assert_eq!(result.stack.length, 0);
    }
}