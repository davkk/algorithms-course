fn bubble_sort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.clone();

    for i in 0..arr.len() {
        for j in i..arr.len() {
            // this is 5head
            if arr[i] > arr[j] {
                // instead of doing:
                // ---
                // let tmp = arr[j];
                // arr[j] = arr[j + 1];
                // arr[j + 1] = tmp;
                // ---
                // you can do:
                arr.swap(i, j);
            }
        }
    }

    arr
}

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

        match &self.tail {
            Some(mut old_tail) => {
                old_tail.next = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
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

fn main() {
    // let numbers = vec![1, 4, 7, 3, 2];
    // println!("initial: {:?}", numbers);
    //
    // println!("bubble: {:?}", bubble_sort(&numbers));

    let mut numbers: Queue<_> = Queue::new();
    numbers.push(1);
    numbers.push(4);
    numbers.push(7);
    // numbers.push(3);
    // numbers.push(3);

    println!("{:?}", numbers.to_vec());
    println!("{:?}", numbers.length)
}
