use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub(crate) enum Device {
    Succeeded,
    Waiting,
}
impl Default for Device {
    fn default() -> Self {
        Self::Waiting
    }
}

pub(crate) struct Devices {
    remaining: usize,
    devices: Vec<Device>,
}

impl Deref for Devices {
    type Target = Vec<Device>;
    fn deref(&self) -> &Self::Target {
        &self.devices
    }
}

impl DerefMut for Devices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.devices
    }
}

impl Devices {
    pub(crate) fn new() -> Self {
        Self {
            remaining: 0,
            //set the capacity to 6000 to avoid reallocations
            devices: Vec::with_capacity(6000),
        }
    }

    pub(crate) fn clear_and_grow(&mut self) {
        for i in 0..self.devices.len() {
            self.devices[i] = Device::default();
        }
        self.devices
            .resize(self.devices.len() + 100, Device::default());
        self.remaining = self.devices.len();
    }

    pub(crate) fn set_successful(&mut self, successful_indices: Vec<usize>) {
        successful_indices.iter().for_each(|&i| {
            self.devices[i] = Device::Succeeded;
        });
        self.remaining -= successful_indices.len();
    }

    /// Get the devices's remaining.
    #[must_use]
    pub(crate) fn remaining(&self) -> usize {
        self.remaining
    }
}
