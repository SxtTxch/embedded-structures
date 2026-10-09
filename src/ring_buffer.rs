pub struct RingBuffer<T, const N: usize> {
    arr: [Option<T>; N],
    read_pointer: usize,
    write_pointer: usize,
    length: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            arr: core::array::from_fn(|_| None),
            read_pointer: 0,
            write_pointer: 0,
            length: 0
        }
    }
    pub fn push(&mut self, data: T) -> () {
        if N == 0 {return;}
        match self.arr[self.write_pointer] {
            Some(_) => {
                if self.write_pointer == self.read_pointer {
                    self.read_pointer = (self.read_pointer + 1) % N;
                }
            },
            _ => {self.length += 1}
        }
        self.arr[self.write_pointer] = Some(data);
        self.write_pointer = (self.write_pointer + 1) % N;
    }
    pub fn pop(&mut self) -> Option<T> {
        if N == 0 {return None;}
        match self.arr[self.read_pointer] {
            Some(_) => {
                self.length -= 1
            },
            _ => {}
        }
        let data: Option<T> = self.arr[self.read_pointer].take();
        self.read_pointer = (self.read_pointer + 1) % N;
        return data;
    }
    pub fn len(&self) -> usize {
        self.length
    }
}
