/// The HX711 can run in three modes:
#[derive(Clone, Copy)]
pub enum Mode {
  /// Chanel A with factor 128 gain
  ChAGain128 = 1,
  /// Chanel B with factor 32 gain
  ChBGain32 = 2,
  /// Chanel A with factor 64 gain
  ChAGain64 = 3,
}

impl Mode {
  pub fn to_gain(&self) -> f32 {
      return match self {
          Mode::ChAGain128 => 128.0,
          Mode::ChBGain32 => 32.0,
          Mode::ChAGain64 => 64.0,
      };
  }
}