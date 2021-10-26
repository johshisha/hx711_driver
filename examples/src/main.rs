use hx711_driver::{hx711, mode};

use std::error::Error;
use rppal::gpio;
use rppal::gpio::Gpio;

const GPIO_DOUT: u8 = 5;
const GPIO_CLK: u8 = 6;

const REFERENCE_UNIT: f32 = 8.0;

struct InputPin {
  pin: gpio::InputPin
}

impl hx711::InputPin for InputPin {
  fn is_high(&mut self) -> bool {
    self.pin.is_high()
  }
}

struct OutputPin {
  pin: gpio::OutputPin
}

impl hx711::OutputPin for OutputPin {
  fn set_high(&mut self) {
    self.pin.set_high()
  }

  fn set_low(&mut self) {
    self.pin.set_low()
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mode = mode::Mode::ChAGain128;

  let mut dout = InputPin {
    pin: Gpio::new()?.get(GPIO_DOUT)?.into_input()
  };
  let mut pd_sck = OutputPin{
    pin:Gpio::new()?.get(GPIO_CLK)?.into_output()
  };

  let mut driver = hx711::Hx711::new(
    &mut dout,
    &mut pd_sck,
    mode,
    REFERENCE_UNIT,
  );

  // Read conversion period data
  loop {
      let value = driver.get_weight();

      println!("{}", value);
      println!("========");
  }
}
