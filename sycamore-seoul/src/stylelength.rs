use crate::*;

/// StyleLength handles css length in either/both pixel or percent format.
/// 
/// # Examples
/// ```
/// let _ = StyleLength::Pixel(20.);
/// let _ = StyleLength::Percent(80.);
/// let _ = StyleLength::PixelPercent(20., 20.); 
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub enum StyleLength {
  #[default] Null,
  Pixel(f64),
  Percent(f64),
  PixelPercent(f64, f64),
}

impl StyleLength {

  pub fn new(len: f64, par_len: f64, to_pixel: bool) -> Self {
    if to_pixel {
      Self::Pixel(len)
    } else {
      Self::Percent(len/par_len*100.)
    }
  }

  /// Is given lengths are bigger than self? (min-limitation check)
  /// 
  pub fn min_check(&self, len: f64, par_len: f64) -> bool {
    match self {
      Self::Pixel(v) => len>=*v,
      Self::Percent(v) => len/par_len*100.>=*v,
      Self::PixelPercent(v1, v2) => len>=*v1 && len/par_len*100.>=*v2,
      _ => true,
    }
  }

  /// Is given lengths are smaller than self? (max-limitation check)
  /// 
  pub fn max_check(&self, len: f64, par_len: f64) -> bool {
    match self {
      Self::Pixel(v) => len<=*v,
      Self::Percent(v) => len/par_len*100.<=*v,
      Self::PixelPercent(v1, v2) => len<=*v1 && len/par_len*100.<=*v2,
      _ => true,
    }
  }

  /// Change self into percent length
  /// 
  pub fn to_percent(&self, parent_len: f64) -> f64 {
    match self {
      Self::Pixel(v) => (*v)/parent_len*100.,
      Self::Percent(v) => *v,
      Self::PixelPercent(_, v) => *v,
      _ => 0.
    }
  }

  fn pixel(&self) -> Option<f64> {
    match self {
      Self::Pixel(v) => Some(*v),
      Self::Percent(_) => None,
      Self::PixelPercent(v, _) => Some(*v),
      _ => None,
    }
  }

  fn percent(&self) -> Option<f64> {
    match self {
      Self::Pixel(_) => None,
      Self::Percent(v) => Some(*v),
      Self::PixelPercent(_, v) => Some(*v),
      _ => None,
    }
  }

  pub fn value(&self) -> f64 {
    match self {
      Self::Pixel(v) => *v,
      Self::Percent(v) => *v,
      Self::PixelPercent(v, _) => *v,
      _ => 0.
    }
  }

  pub fn style_value(&self) -> String {
    if let Some(pixel) = self.pixel() {
      format!("{:.2}px", pixel)
    } else if let Some(percent) = self.percent() {
      format!("{:.2}%", percent)
    } else {
      String::new()
    }
  }

  /// Set style of lengths
  /// 
  pub fn set_style(&self, element: &HtmlElement, is_lateral: bool) {

    if let Self::Null = self {
      //
    } else {
      let property = if is_lateral { "width" } else { "height" };
      element.style().set_property(property, &self.style_value()).unwrap_throw();
    }
  }
}