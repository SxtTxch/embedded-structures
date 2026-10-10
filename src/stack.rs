use crate::fixed_vec::FixedVec;
use core::ops::Deref;
pub struct Stack<T, const N: usize> {
    vec: FixedVec<T, N>,
}
// the heapless stack structure derefs to the inner vec which is an implementation of FixedVec
// ./fixed_vec.rs
impl<T, const N: usize> Deref for Stack<T,N> {
    type Target = FixedVec<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T, const N: usize> Stack<T,N> {
    pub fn new() -> Self {
        Self {
            vec:  FixedVec::new()
        }
    }
    pub fn push(&mut self, data: T) -> Result<(), T> {
        self.vec.push(data)
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.vec.last_mut()
    }
}
