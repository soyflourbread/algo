use std::collections::VecDeque as Deque;

#[derive(Default)]
pub struct SlidingWindow {
    queue: Deque<usize>, // a deque is more expressive

    // TODO: add window's internal states
}

impl SlidingWindow {
    pub fn scan(mut self, token: T) -> (Self, Option<Vec<usize>>) {
        // Add new token to the back of window
        self.queue.push_back(token);
        // TODO: update window's internal states

        // Shrink the window to its minimum size
        while !self.is_minimum() {
            let token_frnt = self.queue.pop_front().unwrap();
            // TODO: update window's internal states
        }

        let token_out = if self.is_valid() {
            Some(self.queue.clone().into_iter().collect::<Vec<_>>())
        } else { None };

        (self, token_out)
    }

    fn is_minimum(&self) -> bool {
        if self.queue.is_empty() { return true; }

        // TODO: check if window is minimum
    }

    fn is_valid(&self) -> bool {
        // TODO: check if window is valid
    }
}
