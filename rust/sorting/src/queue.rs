type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug)]
struct Node<T>
where
    T: Clone,
{
    value: T,
    next: Link<T>,
}

#[derive(Debug)]
struct Queue<T>
where
    T: Clone,
{
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T> Queue<T>
where
    T: Clone + std::fmt::Debug,
{
    fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn push(&mut self, value: T) {
        let new_tail = Box::new(Node { value, next: None });

        self.length += 1;

        match std::mem::replace(&mut self.tail, Some(new_tail)) {
            Some(mut old_tail) => {
                old_tail.next = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail.clone());
            }
        }

        println!("{:?}", &self);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.tail = old_head.next;

            if self.head.is_none() {
                self.tail = None;
            }

            self.length -= 1;

            old_head.value
        })
    }

    fn to_vec(&self) -> Vec<&T> {
        let mut vector = Vec::new();
        let mut curr_node = &self.head;

        while let Some(node) = curr_node {
            println!("{:?}", curr_node);
            vector.push(&node.value);
            curr_node = &node.next;
        }

        vector
    }
}

