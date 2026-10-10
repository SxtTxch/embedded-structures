pub mod fixed_vec;
pub mod ring_buffer;
pub mod stack;

#[cfg(test)]
mod tests {
    use crate::fixed_vec::FixedVec;
    use crate::stack::Stack;

    // ============================================================
    // FIXED_VEC: INITIALIZATION
    // ============================================================

    #[test]
    fn fixed_vec_starts_empty() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(FixedVec::<i32, 4>::capacity(), 4);
        assert_eq!(vec.last(), None);
        assert_eq!(vec.last_mut(), None);
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn fixed_vec_zero_capacity() {
        let mut vec = FixedVec::<i32, 0>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(FixedVec::<i32, 0>::capacity(), 0);
        assert_eq!(vec.push(10), Err(10));
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.last(), None);
        assert_eq!(vec.last_mut(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn fixed_vec_capacity_one() {
        let mut vec = FixedVec::<i32, 1>::new();

        assert_eq!(vec.push(10), Ok(()));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.last(), Some(&10));

        assert_eq!(vec.push(20), Err(20));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
    }

    // ============================================================
    // FIXED_VEC: PUSH
    // ============================================================

    #[test]
    fn fixed_vec_push_one_element() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert_eq!(vec.push(42), Ok(()));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.last(), Some(&42));
    }

    #[test]
    fn fixed_vec_push_multiple_elements() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert_eq!(vec.push(10), Ok(()));
        assert_eq!(vec.push(20), Ok(()));
        assert_eq!(vec.push(30), Ok(()));

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.last(), Some(&30));
    }

    #[test]
    fn fixed_vec_push_to_capacity() {
        let mut vec = FixedVec::<i32, 3>::new();

        assert_eq!(vec.push(10), Ok(()));
        assert_eq!(vec.push(20), Ok(()));
        assert_eq!(vec.push(30), Ok(()));

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.push(40), Err(40));
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.last(), Some(&30));
    }

    #[test]
    fn fixed_vec_failed_push_preserves_original_value() {
        let mut vec = FixedVec::<String, 1>::new();

        assert!(vec.push(String::from("first")).is_ok());

        let rejected = String::from("second");
        assert_eq!(vec.push(rejected.clone()), Err(rejected));

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.last().map(String::as_str), Some("first"));
    }

    #[test]
    fn fixed_vec_failed_push_does_not_modify_contents() {
        let mut vec = FixedVec::<i32, 2>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());

        assert_eq!(vec.push(30), Err(30));
        assert_eq!(vec.len(), 2);

        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
    }

    // ============================================================
    // FIXED_VEC: POP
    // ============================================================

    #[test]
    fn fixed_vec_pop_empty() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert_eq!(vec.pop(), None);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn fixed_vec_pop_lifo_order() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());
        assert!(vec.push(30).is_ok());

        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn fixed_vec_pop_decreases_length() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());
        assert!(vec.push(30).is_ok());

        assert_eq!(vec.len(), 3);
        vec.pop();
        assert_eq!(vec.len(), 2);
        vec.pop();
        assert_eq!(vec.len(), 1);
        vec.pop();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn fixed_vec_reuse_after_pop() {
        let mut vec = FixedVec::<i32, 2>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());

        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.push(30), Ok(()));

        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn fixed_vec_repeated_fill_and_drain() {
        let mut vec = FixedVec::<i32, 3>::new();

        for round in 0..5 {
            for value in 1..=3 {
                assert_eq!(vec.push(round * 10 + value), Ok(()));
            }

            assert_eq!(vec.push(999), Err(999));

            for value in (1..=3).rev() {
                assert_eq!(vec.pop(), Some(round * 10 + value));
            }

            assert_eq!(vec.pop(), None);
            assert_eq!(vec.len(), 0);
        }
    }

    // ============================================================
    // FIXED_VEC: LAST AND LAST_MUT
    // ============================================================

    #[test]
    fn fixed_vec_last_returns_latest_element() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert_eq!(vec.last(), None);

        assert!(vec.push(10).is_ok());
        assert_eq!(vec.last(), Some(&10));

        assert!(vec.push(20).is_ok());
        assert_eq!(vec.last(), Some(&20));
    }

    #[test]
    fn fixed_vec_last_does_not_remove_element() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());

        assert_eq!(vec.last(), Some(&20));
        assert_eq!(vec.last(), Some(&20));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn fixed_vec_last_mut_modifies_element() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());

        if let Some(last) = vec.last_mut() {
            *last = 99;
        }

        assert_eq!(vec.last(), Some(&99));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.pop(), Some(99));
        assert_eq!(vec.pop(), Some(10));
    }

    #[test]
    fn fixed_vec_last_mut_empty() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert_eq!(vec.last_mut(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn fixed_vec_last_after_pop() {
        let mut vec = FixedVec::<i32, 4>::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());
        assert!(vec.push(30).is_ok());

        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.last(), Some(&20));

        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.last(), Some(&10));
    }

    // ============================================================
    // FIXED_VEC: NON-COPY TYPES AND DROP
    // ============================================================

    #[test]
    fn fixed_vec_supports_string() {
        let mut vec = FixedVec::<String, 2>::new();

        assert!(vec.push(String::from("hello")).is_ok());
        assert!(vec.push(String::from("world")).is_ok());

        assert_eq!(vec.pop().as_deref(), Some("world"));
        assert_eq!(vec.pop().as_deref(), Some("hello"));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn fixed_vec_drops_popped_value_exactly_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DropCounter;

        impl Drop for DropCounter {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        DROPS.store(0, Ordering::SeqCst);

        let mut vec = FixedVec::<DropCounter, 2>::new();

        assert!(vec.push(DropCounter).is_ok());
        assert_eq!(DROPS.load(Ordering::SeqCst), 0);

        drop(vec.pop());
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);

        drop(vec);
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn fixed_vec_drops_remaining_elements_on_drop() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DropCounter;

        impl Drop for DropCounter {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        DROPS.store(0, Ordering::SeqCst);

        {
            let mut vec = FixedVec::<DropCounter, 4>::new();

            assert!(vec.push(DropCounter).is_ok());
            assert!(vec.push(DropCounter).is_ok());
            assert!(vec.push(DropCounter).is_ok());

            assert_eq!(DROPS.load(Ordering::SeqCst), 0);
        }

        assert_eq!(DROPS.load(Ordering::SeqCst), 3);
    }

    // ============================================================
    // STACK: INITIALIZATION
    // ============================================================

    #[test]
    fn stack_starts_empty() {
        let stack = Stack::<i32, 4>::new();

        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn stack_zero_capacity() {
        let mut stack = Stack::<i32, 0>::new();

        assert_eq!(stack.len(), 0);
        assert_eq!(stack.push(10), Err(10));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn stack_capacity_one() {
        let mut stack = Stack::<i32, 1>::new();

        assert_eq!(stack.push(10), Ok(()));
        assert_eq!(stack.peek(), Some(&10));

        assert_eq!(stack.push(20), Err(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    // ============================================================
    // STACK: PUSH AND POP
    // ============================================================

    #[test]
    fn stack_push_pop_lifo() {
        let mut stack = Stack::<i32, 4>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());
        assert!(stack.push(30).is_ok());

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_pop_empty() {
        let mut stack = Stack::<i32, 4>::new();

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn stack_rejects_push_when_full() {
        let mut stack = Stack::<i32, 2>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());

        assert_eq!(stack.push(30), Err(30));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&20));
    }

    #[test]
    fn stack_reuse_after_pop() {
        let mut stack = Stack::<i32, 2>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.push(30), Ok(()));

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_repeated_fill_and_drain() {
        let mut stack = Stack::<i32, 3>::new();

        for round in 0..5 {
            for value in 1..=3 {
                assert_eq!(stack.push(round * 10 + value), Ok(()));
            }

            assert_eq!(stack.push(999), Err(999));

            for value in (1..=3).rev() {
                assert_eq!(stack.pop(), Some(round * 10 + value));
            }

            assert_eq!(stack.pop(), None);
            assert_eq!(stack.len(), 0);
        }
    }

    // ============================================================
    // STACK: PEEK AND PEEK_MUT
    // ============================================================

    #[test]
    fn stack_peek_returns_top() {
        let mut stack = Stack::<i32, 4>::new();

        assert_eq!(stack.peek(), None);

        assert!(stack.push(10).is_ok());
        assert_eq!(stack.peek(), Some(&10));

        assert!(stack.push(20).is_ok());
        assert_eq!(stack.peek(), Some(&20));
    }

    #[test]
    fn stack_peek_does_not_remove_top() {
        let mut stack = Stack::<i32, 4>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());

        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn stack_peek_mut_changes_top() {
        let mut stack = Stack::<i32, 4>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());

        if let Some(top) = stack.peek_mut() {
            *top = 99;
        }

        assert_eq!(stack.peek(), Some(&99));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(99));
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn stack_peek_mut_empty() {
        let mut stack = Stack::<i32, 4>::new();

        assert_eq!(stack.peek_mut(), None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn stack_peek_after_pop() {
        let mut stack = Stack::<i32, 4>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());
        assert!(stack.push(30).is_ok());

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.peek(), Some(&20));
    }

    // ============================================================
    // STACK: DEREF
    // ============================================================

    #[test]
    fn stack_deref_exposes_fixed_vec_methods() {
        let mut stack = Stack::<i32, 4>::new();

        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.last(), Some(&20));
    }

    // ============================================================
    // STACK: NON-COPY TYPES
    // ============================================================

    #[test]
    fn stack_supports_string() {
        let mut stack = Stack::<String, 2>::new();

        assert!(stack.push(String::from("first")).is_ok());
        assert!(stack.push(String::from("second")).is_ok());

        assert_eq!(stack.pop().as_deref(), Some("second"));
        assert_eq!(stack.pop().as_deref(), Some("first"));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_returns_owned_value_on_pop() {
        let mut stack = Stack::<String, 2>::new();

        assert!(stack.push(String::from("hello")).is_ok());

        let value = stack.pop();

        assert_eq!(value.as_deref(), Some("hello"));
        assert_eq!(stack.peek(), None);
    }
}

