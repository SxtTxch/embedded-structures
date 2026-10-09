pub struct fixed_vec<T, const N: usize> {
    arr: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> fixed_vec<T, N> {
    pub fn new() -> Self {
        Self { arr: core::array::from_fn(|_| None), len: 0}
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity() -> usize {
        N
    }
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len < N {
            self.arr[self.len] = Some(value);
            self.len += 1;
            Ok(())
        } else { Err(value) }
    }
    pub fn pop(&mut self) -> Option<T> {
        let data: Option<T> = self.arr[N-1].take();
        self.len -= 1;
        return data;
    }
}


