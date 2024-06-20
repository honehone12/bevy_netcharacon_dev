use std::vec::Drain;
use bevy::prelude::*;

#[derive(Component)]
pub struct InstantEventBuffer<E: Event> {
    buff: Vec<E>
}

impl<E: Event> InstantEventBuffer<E> {
    #[inline]
    pub fn new() -> Self {
        Self { 
            buff: Vec::<E>::new() 
        }
    }

    #[inline]
    pub fn send(&mut self, e: E) {
        self.buff.push(e);
    }

    #[inline]
    pub fn read(&mut self) -> Drain<'_, E> {
        self.buff.drain(..)
    }
}
