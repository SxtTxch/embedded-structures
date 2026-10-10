pub struct FixedVec<T, const N: usize> {
    pub arr: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> FixedVec<T, N> {
    pub fn new() -> Self {
        Self { arr: core::array::from_fn(|_| None), len: 0}
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity() -> usize {
        N
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len < N {
            self.len += 1;
            self.arr[self.len-1] = Some(value);
            Ok(())
        } else { Err(value) }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {return None}
        let data: Option<T> = self.arr[self.len-1].take();
        self.len -= 1;
        return data;
    }
    pub fn last(&self) -> Option<&T> {
        if self.len == 0 {return None}
        self.arr[self.len-1].as_ref()
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {return None}
        self.arr[self.len-1].as_mut()
    }
}


