pub use super::utils::{median, delay_us};
pub use super::mode::Mode;

const SETUP_SLEEP: u64 = 70;
const TIME_BEFORE_READOUT: u64 = 1; // T1
const TIME_SCK_HIGH: u64 = 1; // T3
const TIME_SCK_LOW: u64 = 1; // T4

pub trait InputPin {
  fn is_high(&mut self) -> bool;
}

pub trait OutputPin {
  fn set_high(&mut self);
  fn set_low(&mut self);
}

pub struct Hx711<'a, IN, OUT> {
  dout: &'a mut IN,
  pd_sck: &'a mut OUT,
  mode: Mode,
  offset: f32,
  reference_unit: f32,
}

impl<'a, IN, OUT> Hx711<'a, IN, OUT>
  where IN: InputPin, OUT: OutputPin {

  pub fn new(dout: &'a mut IN, pd_sck: &'a mut OUT, mode: Mode, reference_unit: f32) -> Self {
      let mut driver = Hx711 {
          dout,
          pd_sck,
          mode,
          offset: 0.0,
          reference_unit,
      };
      driver.reset();
      return driver;
  }

  fn set_offset(&mut self, offset: &f32) {
      self.offset = *offset;
  }

  pub fn reset(&mut self) {
      // Setup
      self.pd_sck.set_high();
      delay_us(SETUP_SLEEP);
      self.pd_sck.set_low();

      // Set to offset for zero value
      let zero_value = self.get_weight();
      self.set_offset(&zero_value);
  }

  pub fn get_weight(&mut self) -> f32 {
      const TIMES: usize = 5;
      let mut results: [f32; TIMES] = Default::default();
      for n in 0..TIMES {
          results[n] = self.retrieve();
      }
      println!("{:?}", results);
      return median(&mut results.to_vec()) - self.offset;
  }

  pub fn retrieve(&mut self) -> f32 {
      while self.dout.is_high() {
          delay_us(1);
      }
      self.pd_sck.set_low();
      delay_us(TIME_BEFORE_READOUT);

      let mut count: i32 = 0;
      for _ in 0..24 {
          // Read 24 bits
          count <<= 1;
          self.pd_sck.set_high();
          delay_us(TIME_SCK_HIGH / 2);

          if self.dout.is_high() {
              count += 1;
          }

          delay_us(TIME_SCK_HIGH / 2);

          self.pd_sck.set_low();
          delay_us(TIME_SCK_LOW);
      }

      // Continue to set mode for next conversion
      let n_reads = self.mode as u16;
      for _ in 0..n_reads {
          self.pd_sck.set_high();
          delay_us(TIME_SCK_HIGH);
          self.pd_sck.set_low();
          delay_us(TIME_SCK_LOW);
      }

      return count as f32 / self.mode.to_gain() / self.reference_unit;
  }
}
