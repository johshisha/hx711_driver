use std::thread;
use std::time::Duration;

pub fn median(numbers: &mut Vec<f32>) -> f32 {
  numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let mid = numbers.len() / 2;
  numbers[mid]
}

pub fn delay_us(micros: u64) {
  thread::sleep(Duration::from_micros(micros));
}