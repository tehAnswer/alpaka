use std::fmt::{Display, Formatter, Result};

pub enum AlpakaMode {
  Paper,
  Live,
}

impl Display for AlpakaMode {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      AlpakaMode::Paper => "paper-api",
      AlpakaMode::Live => "live",
    };

    write!(f, "{}", value)
  }
}

#[cfg(test)]
mod tests {
  use crate::AlpakaMode;

  #[test]
  pub fn alpaka_mode_paper_to_string() {
    assert_eq!(&AlpakaMode::Paper.to_string(), "paper-api")
  }

  #[test]
  pub fn alpaka_mode_live_to_string() {
    assert_eq!(&AlpakaMode::Live.to_string(), "live")
  }
}
