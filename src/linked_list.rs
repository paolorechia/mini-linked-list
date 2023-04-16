//! # Linked List
//! 
//! Implements a simple linked list using Heap memory.
//! Currently, there's an implementation only for the i32 primitive type.

use core::ptr::NonNull;


pub struct LinkedList<T> 
    where T: Copy,
{
    pub val: Option<T>,
    pub next: Option<NonNull<LinkedList<T>>>,
}

impl<T> LinkedList<T>
where T: Copy,
 {
    
    /// # Creates an empty LinkedList that may hold i32 values
    /// 
    /// # Example
    /// ```
    /// let list = mini_linked_list::LinkedList::<&str>::new();
    /// ```
    pub fn new() -> LinkedList<T>{
        LinkedList {
            val: None,
            next: None
        }
    }
    /// # Adds an element to the left side of the list
    /// 
    /// This method works in O(1) operation, as it replaces the head of the list
    /// with a new one and no traversal thus is required.
    /// 
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
    pub fn push_left(&mut self, x: T) {
        // allocate on the heap
        let node = Box::new(LinkedList::<T> {
            next: None,
            val: Some(x)
        });
        // take control over the raw pointer
        if self.next.is_none() {
            // our list is empty
            // allocate on the heap
            // head is now the new pointer
            let pter: NonNull<LinkedList<T>> = Box::leak(node).into();
            self.next = Some(pter);

        } else {
            // our list already has an element
            let mut pter: NonNull<LinkedList<T>> = Box::leak(node).into();
            unsafe {
                // new node should point to current head
                pter.as_mut().next = self.next;
            }
            // head is now the new pointer
            self.next = Some(pter);

        }
    }

    pub fn pop_left(&mut self) -> Option<T> {
        // empty list
        if self.next.is_none() {
            return None
        } else {
            let mut next = self.next.unwrap();

            // list only has one element
            let only_one: bool = unsafe {next.as_mut().next.is_none()};
            if only_one == true {
                // pull the next element from the raw pointer
                // and store it in a box, so memory free can work correctly
                let next_box = unsafe {
                    Box::from_raw(next.as_ptr())
                };
                self.next = None;
                next_box.val
            }
            // list has two or more elements
            else {
                let next_of_next = unsafe { next.as_mut().next };
                let next_box = unsafe {
                    Box::from_raw(next.as_ptr())
                };
                self.next = next_of_next;
                next_box.val
            }
        }
    }


    pub fn collect(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();
        if self.next.is_none() {
            return result
        }

        let mut node = self.next.unwrap();
        unsafe {
            result.push(node.as_mut().val.unwrap());
            while node.as_mut().next.is_some() {
                node = node.as_mut().next.unwrap();
                result.push(node.as_mut().val.unwrap());
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

    #[test]
    fn test_linked_list_pop_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        assert_eq!(list.collect(), vec![]);

        list.push_left(1);
        list.push_left(2);
        list.push_left(3);
        list.push_left(4);
        assert_eq!(list.collect(), vec![4,3,2,1]);

        let x = list.pop_left();
        assert_eq!(x.unwrap(), 4);

        let x = list.pop_left();
        assert_eq!(x.unwrap(), 3);

        let x = list.pop_left();
        assert_eq!(x.unwrap(), 2);

        let x = list.pop_left();
        assert_eq!(x.unwrap(), 1);

        let x = list.pop_left();
        assert_eq!(x.is_none(), true);
    }
}