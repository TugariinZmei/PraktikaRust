struct MyVector<T> {
    data: Box<[Option<T>]>,
    size: usize,
}

impl<T: std::clone::Clone> MyVector<T> {
    pub fn new() -> Self {
        MyVector {
            data: Box::new([]),
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }

        MyVector {
            data: data.into_boxed_slice(),
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.size >= self.data.len() {
            self.resize(self.size + 1);
        }

        self.data[self.size] = Some(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        self.data[self.size].take()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let value = self.data[index].take();

        for i in index..self.size - 1 {
            self.data[i] = self.data[i + 1].take();
        }

        self.size -= 1;

        value
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        self.data[index].as_ref()
    }

    pub fn resize(&mut self, new_size: usize) {
        let mut new_data = Vec::with_capacity(new_size);
        new_data.extend_from_slice(&self.data[..self.size]);

        for _ in self.size..new_size {
            new_data.push(None);
        }

        self.data = new_data.into_boxed_slice();
        self.size = new_size;
    }
}

fn main() {
    let mut vector: MyVector<i32> = MyVector::with_capacity(3); // Set initial capacity

    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("Size: {}", vector.size);  // Output: Size: 3

    while let Some(value) = vector.pop() {
        println!("Popped: {}", value);
    }
}

