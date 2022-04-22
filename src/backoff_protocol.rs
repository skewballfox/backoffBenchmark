use crate::devices::{Device, Devices};
use crate::window::{Slot, Window};
use rand::prelude::*;

pub struct BackoffProtocol<F>
where
    F: Fn(usize) -> usize,
{
    devices: Devices,
    initial_window_size: usize,
    window: Window<F>,
}

impl<F> BackoffProtocol<F>
where
    F: Fn(usize) -> usize,
{
    pub fn new(initial_window_size: usize, growth_rate: F) -> Self {
        BackoffProtocol {
            devices: Devices::new(),
            initial_window_size,
            window: Window::new(initial_window_size, growth_rate),
        }
    }

    pub fn run_experiment(&mut self) -> usize {
        let mut rng = thread_rng();

        //since we never want to accidentally start with state, I figured it would be best to move
        //this to preprocessing within the experiment function
        self.devices.clear_and_grow();
        //p.s. this only does work if window size greater than initial window size
        self.window.truncate(self.initial_window_size);

        let mut latency = 0;

        loop {
            let num_slots = self.window.len();
            //first iterate through the devices and select a random slot for each.

            (0..self.devices.len()).for_each(|device_i| {
                if let Device::Waiting = self.devices[device_i] {
                    let slot_i = rng.gen_range(0..num_slots);

                    self.window.insert(device_i, slot_i);
                }
            });
            //then collect the succesful slots and clear the window state
            let successes: Vec<usize> = self
                .window
                .iter_mut()
                .map(|slot| slot.clear())
                .flatten()
                .collect();

            //set the successful devices and check how many still need to transmit
            self.devices.set_successful(successes.clone());
            //if finished add last index to latency and break
            if self.devices.remaining() == 0 {
                let last_index = successes.iter().max().unwrap();
                latency += *last_index + 1;
                //THIS IS VALID FUCKING SYNTAX
                break latency;
            } else {
                //otherwise grow and clear the window and add the previous window size to the total latency
                latency += self.window.grow();
            }
        }
    }
}
