use std::{
    cell::RefMut,
    io::Empty,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Slot {
    Empty,
    Occupied(usize),
    Collision,
}
impl Default for Slot {
    fn default() -> Self {
        Self::Empty
    }
}

impl Slot {
    fn insert(self, device_index: usize) -> Slot {
        match self {
            Slot::Empty => Slot::Occupied(device_index),
            _ => Slot::Collision,
        }
    }

    pub(crate) fn clear(&mut self) -> Option<usize> {
        let x = match self {
            Slot::Occupied(x) => Some(*x),
            _ => None,
        };
        *self = Slot::default();
        return x;
    }
}

pub(crate) struct Window<F>
where
    F: Fn(usize) -> usize,
{
    slots: Vec<Slot>,
    growth_rate: F,
}

impl<F> Window<F>
where
    F: Fn(usize) -> usize,
{
    pub(crate) fn new(initial_window_size: usize, growth_rate: F) -> Self {
        Self {
            slots: vec![Slot::default(); initial_window_size],
            growth_rate,
        }
    }
    /// grows the function according to the rate of growth
    /// returns the length of the window prior to growth which is
    /// used to calculate the latency of a given backoff protocol
    pub(crate) fn grow(&mut self) -> usize {
        let x = self.len();
        self.slots
            .resize((self.growth_rate)(self.slots.len()), Slot::default());
        println!("old len: {} new len: {}", x, self.slots.len());
        x
    }

    pub(crate) fn insert(&mut self, device_i: usize, slot_i: usize) {
        let slot = self.slots[slot_i].insert(device_i);
        self.slots[slot_i] = slot;
    }

    /// Set the window's slots.
    pub(crate) fn set_slots(&mut self, initial_window_size: usize) {
        self.slots = vec![Slot::default(); initial_window_size];
    }
}

impl<F> Deref for Window<F>
where
    F: Fn(usize) -> usize,
{
    type Target = Vec<Slot>;
    fn deref(&self) -> &Self::Target {
        &self.slots
    }
}

impl<F> DerefMut for Window<F>
where
    F: Fn(usize) -> usize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slots
    }
}
