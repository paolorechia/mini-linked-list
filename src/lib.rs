//! # Linked List
//! 
//! Implements a simple linked list using Heap memory.

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}

impl LinkedList<i32> {
    pub fn new() -> LinkedList<i32>{
        LinkedList {
            val: None,
            next: None,
        }
    }
    pub fn push(&mut self, x: i32) {
        let mut node = self;
        while node.next.is_some() {
            node = node.next.as_mut().unwrap();
        }
        node.next = Some(Box::new(LinkedList{
            val: Some(x),
            next: None,
        }))
    }

    pub fn collect(&self) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        let mut node = self;
        while node.next.is_some() {
            match node.val {
                Some(val) => result.push(val),
                _ => ()
            }
            node = node.next.as_ref().unwrap();
        }
        match node.val {
            Some(val) => result.push(val),
            _ => ()
        }

        return result;

    }
}

// Couldn't figure out how to implement this yet
// For now we have an iteration that copies into a vector to collect (not ideal)
// impl <'a>Iterator for LinkedList<i32> {
//     type Item = &'a Box<LinkedList<i32>>;

//     fn next<'a> (self: &'a mut LinkedList<i32>) -> Option<Box<LinkedList<i32>>>{
//         match &self.next {
//             Some(value) => Some(value),
//             None => None
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.collect(), vec![1,2,3,4]);
    }
}