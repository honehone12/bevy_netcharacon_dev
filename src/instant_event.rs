use std::collections::{VecDeque, vec_deque::Drain};
use bevy::prelude::*;

#[derive(Resource)]
pub struct InstantEvent<E: Event> {
    deq: VecDeque<E>
}

impl<E: Event> InstantEvent<E> {
    #[inline]
    pub fn new() -> Self {
        Self { 
            deq: VecDeque::<E>::new() 
        }
    }

    #[inline]
    pub fn enqueue(&mut self, e: E) {
        self.deq.push_back(e);
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<'_, E> {
        self.deq.drain(..)
    }
}
