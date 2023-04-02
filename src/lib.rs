//! # Linked List
//! 
//! Implements a simple linked list using Heap memory.

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}
pub struct PopLeftResult<T> {
    pub val: Option<T>,
    pub list: Option<Box<LinkedList<T>>>
}

impl LinkedList<i32> {
    pub fn new() -> LinkedList<i32>{
        LinkedList {
            val: None,
            next: None,
        }
    }
    pub fn push_right(&mut self, x: i32) {
        let mut node = self;
        while node.next.is_some() {
            node = node.next.as_mut().unwrap();
        }
        node.next = Some(Box::new(LinkedList{
            val: Some(x),
            next: None,
        }))
    }

    pub fn push_left(self, x: i32) -> LinkedList<i32>{
        let node= LinkedList::<i32> {
            val: Some(x),
            next: Some(Box::new(self))
        };
        node
    }

    pub fn pop_left(self) -> PopLeftResult<i32> {
        if self.val.is_some() {
            return PopLeftResult { val: self.val, list: self.next }
        }
        match self.next {
            Some(node) => PopLeftResult { val: node.val, list: node.next },
            None => PopLeftResult { val: self.val, list: None }
        }
    }

    pub fn collect(&self) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        let mut node = self;

        match node.val {
            Some(val) => result.push(val),
            _ => ()
        }
        while node.next.is_some() {
            node = node.next.as_ref().unwrap();
            match node.val {
                Some(val) => result.push(val),
                _ => ()
            }
        }

        return result;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_push_right() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_right(1);
        list.push_right(2);
        list.push_right(3);
        list.push_right(4);
        assert_eq!(list.collect(), vec![1,2,3,4]);
    }
    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list = list.push_left(1);
        list = list.push_left(2);
        list = list.push_left(3);
        list = list.push_left(4);
        assert_eq!(list.collect(), vec![4,3,2,1]);
    }
    
    #[test]
    fn test_linked_list_pop_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_right(1);
        list.push_right(2);
        list.push_right(3);
        list.push_right(4);

        let result = list.pop_left();
        let list = result.list.unwrap();

        assert_eq!(list.collect(), vec![2,3,4]);
        assert_eq!(result.val.unwrap(), 1);

        let result = list.pop_left();
        let list = result.list.unwrap();

        assert_eq!(list.collect(), vec![3,4]);
        assert_eq!(result.val.unwrap(), 2);

        let result = list.pop_left();
        let list = result.list.unwrap();

        assert_eq!(list.collect(), vec![4]);
        assert_eq!(result.val.unwrap(), 3);

        let result = list.pop_left();
        assert_eq!(result.list.is_none(), true);
        assert_eq!(result.val.unwrap(), 4);

    }
}