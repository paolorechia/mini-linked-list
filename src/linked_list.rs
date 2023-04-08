//! # Linked List
//! 
//! Implements a simple linked list using Heap memory.
//! Currently, there's an implementation only for the i32 primitive type.

use core::ptr::NonNull;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    pub next: Option<NonNull<Node<T>>>,
    pub element: T,
}

impl LinkedList<i32> {
    /// # Creates an empty LinkedList that may hold i32 values
    /// 
    /// # Example
    /// ```
    /// let list = mini_linked_list::LinkedList::<i32>::new();
    /// ```
    pub fn new() -> LinkedList<i32>{
        LinkedList {
            head: None
        }
    }
    /// # Adds an element to the left side of the list
    /// 
    /// This method works in O(1) operation, as it replaces the head of the list
    /// with a new one and no traversal thus is required.
    /// 
    /// The method returns the new memory address of the list that must be handled
    /// by the caller (typically reassigning the variable).
    /// 
    /// # Example
    /// ```
    /// use mini_linked_list::LinkedList;
    /// let mut list: LinkedList<i32> = LinkedList::<i32>::new();
    /// list.push_left(1);
    /// list.push_left(2);
    /// list.push_left(3);
    /// list.push_left(4);
    /// ```
    pub fn push_left(&mut self, x: i32) {
        unsafe {
            // allocate on the heap
            let node = Box::new(Node::<i32> {
                next: None,
                element: x
            });
            // take control over the raw pointer
            let mut pter: NonNull<Node<i32>> = Box::leak(node).into();
            // update the next to point to head
            pter.as_mut().next = self.head;
            // head is now the new pointer
            self.head = Some(pter);
        }
    }

    pub fn collect(&self) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        if self.head.is_none() {
            return result
        }

        let mut node = self.head.unwrap();
        unsafe {
            result.push(node.as_mut().element);
            while node.as_mut().next.is_some() {
                node = node.as_mut().next.unwrap();
                result.push(node.as_mut().element);
            }
        }
        return result;

    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        assert_eq!(list.collect(), vec![]);

        list.push_left(1);
        list.push_left(2);
        list.push_left(3);
        list.push_left(4);
        assert_eq!(list.collect(), vec![4,3,2,1]);
    }
  
}