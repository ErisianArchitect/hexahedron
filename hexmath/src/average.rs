use std::{collections::VecDeque, time::Duration};

pub struct AverageBuffer<T> {
    buffer: VecDeque<T>,
    current_total: T,
}

pub trait AvgBuffer<T> {
    fn new<I: Into<Option<T>>>(capacity: usize, initial: I) -> Self;
    fn push(&mut self, value: T) -> T;
    fn average(&self) -> T;
    fn clear(&mut self);
}

impl AvgBuffer<f32> for AverageBuffer<f32> {
    fn new<I: Into<Option<f32>>>(capacity: usize, initial: I) -> Self {
        if let Some(initial) = initial.into() {
            let mut buffer = VecDeque::with_capacity(capacity);
            buffer.push_back(initial);
            Self {
                buffer,
                current_total: initial,
            }
        } else {
            Self {
                buffer: VecDeque::with_capacity(capacity),
                current_total: 0.0,
            }
        }
    }

    fn push(&mut self, value: f32) -> f32 {
        if self.buffer.len() == self.buffer.capacity() {
            if let Some(front) = self.buffer.pop_front() {
                self.current_total -= front;
            }
        }
        self.buffer.push_back(value);
        self.current_total += value;
        self.current_total / self.buffer.len() as f32
    }

    fn average(&self) -> f32 {
        if self.buffer.is_empty() {
            self.current_total
        } else {
            self.current_total / self.buffer.len() as f32
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.current_total = 0.0;
    }
}

impl AvgBuffer<f64> for AverageBuffer<f64> {
    fn new<I: Into<Option<f64>>>(capacity: usize, initial: I) -> Self {
        if let Some(initial) = initial.into() {
            let mut buffer = VecDeque::with_capacity(capacity);
            buffer.push_back(initial);
            Self {
                buffer,
                current_total: initial,
            }
        } else {
            Self {
                buffer: VecDeque::with_capacity(capacity),
                current_total: 0.0,
            }
        }
    }

    fn push(&mut self, value: f64) -> f64 {
        if self.buffer.len() == self.buffer.capacity() {
            if let Some(front) = self.buffer.pop_front() {
                self.current_total -= front;
            }
        }
        self.buffer.push_back(value);
        self.current_total += value;
        self.current_total / self.buffer.len() as f64
    }

    fn average(&self) -> f64 {
        if self.buffer.is_empty() {
            self.current_total
        } else {
            self.current_total / self.buffer.len() as f64
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.current_total = 0.0;
    }
}

impl AvgBuffer<Duration> for AverageBuffer<Duration> {
    fn new<I: Into<Option<Duration>>>(capacity: usize, initial: I) -> Self {
        if let Some(initial) = initial.into() {
            let mut buffer = VecDeque::with_capacity(capacity);
            buffer.push_back(initial);
            Self {
                buffer,
                current_total: initial,
            }
        } else {
            Self {
                buffer: VecDeque::with_capacity(capacity),
                current_total: Duration::ZERO,
            }
        }
    }

    fn push(&mut self, value: Duration) -> Duration {
        if self.buffer.len() == self.buffer.capacity() {
            if let Some(front) = self.buffer.pop_front() {
                self.current_total -= front;
            }
        }
        self.buffer.push_back(value);
        self.current_total += value;
        self.current_total / self.buffer.len() as u32
    }

    fn average(&self) -> Duration {
        if self.buffer.is_empty() {
            self.current_total
        } else {
            self.current_total / self.buffer.len() as u32
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.current_total = Duration::ZERO;
    }
}