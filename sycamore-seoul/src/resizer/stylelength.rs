/// StyleLength handles css length in pixel and percent type.
/// 
/// # Examples
/// ```
/// let _ = StyleLength::Pixel(20.);
/// let _ = StyleLength::Percent(80.);
/// let _ = StyleLength::PixelPercent(20., 20.); 
/// ```
#[derive(Debug, Clone, Copy)]
pub enum StyleLength {
  Pixel(f64),
  Percent(f64),
  PixelPercent(f64, f64),
}

impl StyleLength {

  /// Check if the given length is larger than min limit.
  pub fn check_min_limit(&self, to_length: f64, to_percent_length: f64) -> bool {
    match self {
      Self::Pixel(v) => to_length>=*v,
      Self::Percent(v) => to_percent_length>=*v,
      Self::PixelPercent(v1, v2) => to_length>=*v1 && to_percent_length>=*v2,
    }
  }

  /// Check if the given length is smaller than max limit.
  pub fn check_max_limit(&self, to_length: f64, to_percent_length: f64) -> bool {
    match self {
      Self::Pixel(v) => to_length<=*v,
      Self::Percent(v) => to_percent_length<=*v,
      Self::PixelPercent(v1, v2) => to_length<=*v1 && to_percent_length<=*v2,
    }
  }
}