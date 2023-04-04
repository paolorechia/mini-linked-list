//! # Linked List
//! 
//! Implements a simple linked list using Heap memory.
//! Currently, there's an implementation only for the i32 primitive type.
pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}
pub struct PopLeftResult<T> {
    pub val: Option<T>,
    pub list: Option<Box<LinkedList<T>>>
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
            val: None,
            next: None,
        }
    }
    /// # Adds an element to the right side of the list
    /// 
    /// This method uses O(N) operations, as it needs to traverse
    /// the entire list before appending the new value to the end of the list.
    /// 
    /// # Example
    /// ```
    /// let mut list = mini_linked_list::LinkedList::<i32>::new();
    /// list.push_right(1);
    /// list.push_right(2);
    /// list.push_right(3);
    /// list.push_right(4);
    /// assert_eq!(list.collect(), vec![1,2,3,4]);
    /// ```
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
    /// list = list.push_left(1);
    /// list = list.push_left(2);
    /// list = list.push_left(3);
    /// list = list.push_left(4);
    /// assert_eq!(list.collect(), vec![4,3,2,1]);
    /// ```
    pub fn push_left(self, x: i32) -> LinkedList<i32>{
        let node= LinkedList::<i32> {
            val: Some(x),
            next: Some(Box::new(self))
        };
        node
    }

    /// # Pops the List head on the left side, returning a PopLeftResult
    /// 
    /// This operation works in O(1), as it only pops the head and
    /// no traversal is required.
    /// 
    /// It's usage is not so straightforward, however, as it requires
    /// the caller to replace the reference to the list head with the
    /// address returned by this method, inside `PopLeftResult.list`
    /// 
    /// It's advised to not call `unwrap` directly in `PopLeftResult.list``
    /// directly, but rather rely on safer `Option` methods.
    /// 
    /// # Example
    /// 
    /// ```
    /// use mini_linked_list::{LinkedList, PopLeftResult};
    /// let mut list: LinkedList<i32> = LinkedList::<i32>::new();
    /// list.push_right(1);
    /// list.push_right(2);
    /// 
    /// let result: PopLeftResult<i32> = list.pop_left();
    /// let list = result.list.unwrap();
    /// 
    /// assert_eq!(list.collect(), vec![2]);
    /// assert_eq!(result.val.unwrap(), 1);
    /// 
    /// let result: PopLeftResult<i32> = list.pop_left();
    /// let list = result.list;
    /// 
    /// assert_eq!(list.is_none(), true);
    /// assert_eq!(result.val.unwrap(), 2);
    /// ```

    pub fn pop_left(self) -> PopLeftResult<i32> {
        if self.val.is_some() {
            return PopLeftResult { val: self.val, list: self.next }
        }
        match self.next {
            Some(node) => PopLeftResult { val: node.val, list: node.next },
            None => PopLeftResult { val: self.val, list: None }
        }
    }

    /// # Pops the List head on the right side.
    /// 
    /// This operation works in O(N), as it requires a full traversal
    /// of the list.
    /// 
    /// Whenever possible, prefer relying on the `pop_left` method, as it is more
    /// efficient.
    /// 
    /// # Example
    /// 
    /// ```
    /// use mini_linked_list::LinkedList;
    /// let mut list: LinkedList<i32> = LinkedList::<i32>::new();
    /// list.push_right(1);
    /// list.push_right(2);
    /// assert_eq!(list.pop_right().unwrap(), 2);
    /// assert_eq!(list.pop_right().unwrap(), 1);
    /// assert_eq!(list.pop_right().is_none(), true);
    /// ```
    pub fn pop_right(&mut self) -> Option<i32>{
        let mut node: &mut Option<Box<LinkedList<i32>>> = &mut self.next;
        let x: Option<i32>;
        // no next node, need to try to pop self
        if node.is_none() {
            let val = self.val;
            self.val = None;
            return val
        }
        // only one next node, we should return it
        if node.as_mut().unwrap().next.is_none() {
            x = node.as_mut().unwrap().val;
            node.as_mut().unwrap().next = None;
            node.as_mut().unwrap().val = None;
            return x;
        }
        // we can assume we have at least two valid next aheads
        else {
            while node.as_mut().unwrap().next.as_mut().unwrap().next.is_some() {
                node = &mut node.as_mut().unwrap().next;
            }
            x = node.as_mut().unwrap().next.as_mut().unwrap().val;
            node.as_mut().unwrap().next = None;
        }
        return x
    }

    /// # Collects the list into an Array
    /// 
    /// This method is used mostly for debugging and testing,
    /// but can also be used to iterate over the list without popping
    /// it's values.
    /// 
    /// It's not memory efficient, however, as it copies the entire
    /// data.
    /// 
    /// # Example
    /// ```
    /// use mini_linked_list::LinkedList;
    /// let mut list: LinkedList<i32> = LinkedList::<i32>::new();
    /// list.push_right(1);
    /// list.push_right(2);
    /// assert_eq!(list.collect(), vec![1,2]);
    /// ```
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

    #[test]
    fn test_linked_list_pop_right() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();

        // case 1
        list.push_right(1);
        assert_eq!(list.pop_right().unwrap(), 1);
        assert_eq!(list.pop_right().is_none(), true);

        // case 2
        list.push_right(1);
        list.push_right(2);

        assert_eq!(list.pop_right().unwrap(), 2);
        assert_eq!(list.pop_right().unwrap(), 1);
        assert_eq!(list.pop_right().is_none(), true);


        // case 3
        list.push_right(1);
        list.push_right(2);
        list.push_right(3);
        assert_eq!(list.pop_right().unwrap(), 3);
        assert_eq!(list.pop_right().unwrap(), 2);
        assert_eq!(list.pop_right().unwrap(), 1);
        assert_eq!(list.pop_right().is_none(), true);

    }

    #[test]
    fn test_several_apis() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_right(1);
        list.push_right(2);
        list.push_right(3);
        list.push_right(4);
        assert_eq!(list.collect(), vec![1,2,3,4]);

        list = list.push_left(1);
        list = list.push_left(2);
        list = list.push_left(3);
        list = list.push_left(4);
        assert_eq!(list.collect(), vec![4,3,2,1,1,2,3,4]);
        let x = list.pop_right();
        assert_eq!(x.unwrap(), 4);
        assert_eq!(list.collect(), vec![4,3,2,1,1,2,3]);

        let result = list.pop_left();
        let list = result.list.unwrap();
        assert_eq!(list.collect(), vec![3,2,1,1,2,3]);
        assert_eq!(result.val.unwrap(), 4);
    }

}