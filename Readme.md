# Heap Linked List in Rust

Learning Rust by implementing simple data structures

## Install

Add to your Cargo.toml dependencies:

```
[dependencies]
mini-linked-list = "0.1.2"
```

## Usage:

### Push right
**Adds an element to the right side of the list**

This method uses O(N) operations, as it needs to traverse
the entire list before appending the new value to the end of the list.

```rust
use mini_linked_list::LinkedList;
let mut list: LinkedList<i32> = LinkedList::<i32>::new();
list.push_right(1);
list.push_right(2);
assert_eq!(list.collect(), vec![1,2]);
```

### Push left
**Adds an element to the left side of the list**

This method works in O(1) operation, as it replaces the head of the list
with a new one and no traversal thus is required.

The method returns the new memory address of the list that must be handled
by the caller (typically reassigning the variable).

```rust
use mini_linked_list::LinkedList;
let mut list: LinkedList<i32> = LinkedList::<i32>::new();
list = list.push_left(1);
list = list.push_left(2);
list = list.push_left(3);
list = list.push_left(4);
assert_eq!(list.collect(), vec![4,3,2,1]);
```

### Pop left
**Pops the List head on the left side, returning a PopLeftResult**

This operation works in O(1), as it only pops the head and
no traversal is required.

It's usage is not so straightforward, however, as it requires
the caller to replace the reference to the list head with the
address returned by this method, inside `PopLeftResult.list`

It's advised to not call `unwrap` directly in `PopLeftResult.list``
directly, but rather rely on safer `Option` methods.

```rust
use mini_linked_list::{LinkedList, PopLeftResult};
let mut list: LinkedList<i32> = LinkedList::<i32>::new();
list.push_right(1);
list.push_right(2);

let result: PopLeftResult<i32> = list.pop_left();
let list = result.list.unwrap();

assert_eq!(list.collect(), vec![2]);
assert_eq!(result.val.unwrap(), 1);

let result: PopLeftResult<i32> = list.pop_left();
let list = result.list;

assert_eq!(list.is_none(), true);
assert_eq!(result.val.unwrap(), 2);
```

### Pop right
**Pops the List head on the right side.**

This operation works in O(N), as it requires a full traversal
of the list.

Whenever possible, prefer relying on the `pop_left` method, as it is more
efficient.

```rust
use mini_linked_list::LinkedList;
let mut list: LinkedList<i32> = LinkedList::<i32>::new();
list.push_right(1);
list.push_right(2);
assert_eq!(list.pop_right().unwrap(), 2);
assert_eq!(list.pop_right().unwrap(), 1);
assert_eq!(list.pop_right().is_none(), true);
```
