# Find In String

The compiler says that the `find_in_string` function is missing a lifetime
specifier. This means that the function's return type contains a borrowed value,
but the signature does not say which parameter it is borrowed from. When we drop
the word variable, the compiler does not know whether or not it should also drop
the value returned from the function. Using the `'a` annotation, we can tell the
compiler that the lifetime of the function's return is connected to the lifetime
of the sentence parameter, so the return is dropped when sentence is dropped and
not when word is dropped.

# Doubly Linked List

In a doubly linked list, the push implementation would have to look something
like this:

```
fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
            prev: None,
        });
        self.head.as_mut().map(|node| node.prev = Some(new_node));
        self.head = Some(new_node);
    }
```

The compiler refuses to compile this code because the `new_node` variable is
moved to the next node's `prev` field. In the next line, we then try to set
`self.head` to the same value, but the value is not at the same memory location
anymore. We cannot use use a value after it is moved, and the compiler throws a
**use of moved value** error.

Another example of something in Rust that would be annoying to do is have two
people own the same car and have to figure out how to share it and who can drive
it when. This would not be possible in safe Rust because only one person could
own the car.
