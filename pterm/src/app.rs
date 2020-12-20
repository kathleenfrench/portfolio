#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;

use crate::PORTFOLIO_SECTIONS;

#[cfg(not(target_arch = "wasm32"))]
fn parse_min_char(s: &str) -> Result<u32, String> {
  let val = s.parse::<u32>().map_err(|e| e.to_string())?;

  if val == 0 {
    return Err("must be greater than 0".to_string());
  }

  Ok(val)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_speed_factor(s: &str) -> Result<f32, String> {
  let value_as_float = s.parse::<f32>().map_err(|e| e.to_string())?;

  if value_as_float < 0.01 {
    return Err("speed factor must be larger than 0.01".to_string());
  }

  Ok(value_as_float)
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(StructOpt)]
#[structopt(
  name = "pterm",
  author,
  about,
  global_settings = &[structopt::clap::AppSettings::ColoredHelp],
)]
pub struct AppConfig {
  /// output a list of the available sections
  #[structopt(short, long = "list-sections")]
  pub list_sections_and_quit: bool,

  /// these are the sections to show as part of the portfolio
  #[structopt(short, long, possible_values = &PORTFOLIO_SECTIONS)]
  pub sections: Vec<String>,

  /// quit after the session has been open for x amount of time
  #[structopt(long, parse(try_from_str = humantime::parse_duration))]
  pub session_timeout: Option<instant::Duration>,

  /// quit after this many sections have played
  #[structopt(long, parse(try_from_str = parse_min_char))]
  pub exit_on_section_max: Option<u32>,

  #[structopt(short, long, default_value = "1", parse(try_from_str = parse_speed_factor))]
  pub speed_factor: f32,
}

#[cfg(target_arch = "wasm32")]
pub struct AppConfig {
  pub sections: Vec<String>,

  pub speed_factor: f32,
}

impl AppConfig {
  pub fn should_quit(&self) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
      use crate::{SECTIONS_SHOWN, START_TIME};
      use std::sync::atomic::Ordering;

      if let Some(since) = self.session_timeout {
        if START_TIME.elapsed() > since {
          return true;
        }
      };

      if let Some(shown) = self.exit_on_section_max {
        if SECTIONS_SHOWN.load(Ordering::SeqCst) >= shown {
          return true;
        }
      };
    }

    false
  }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_inputs() -> AppConfig {
  let mut args = AppConfig::from_args();

  if args.sections.is_empty() {
    args.sections = PORTFOLIO_SECTIONS.iter().map(|x| x.to_string()).collect();
  };

  args
}

#[cfg(target_arch = "wasm32")]
pub fn parse_inputs() -> AppConfig {
  use url::Url;

  let mut sections = vec![];
  let window = web_sys::window().expect("no global `window` exists");
  let loc = window.location();
  let valid_url = Url::parse(&loc.href().unwrap()).unwrap();
  let mut pairs = valid_url.query_pairs();
  let sections_collection = pairs.filter(|&(ref k, _)| k == "section");
  for (_, query) in sections_collection {
    let actual = &&*query;
    if PORTFOLIO_SECTIONS.contains(actual) {
      sections.push(actual.to_string());
    }
  }

  let speed_factor: f32 = pairs
      .find(|&(ref k, _)|k == "speed-factor")
      .map(|(_, v)| v.parse::<f32>().unwrap_or(1.0))
      .unwrap_or(1.0);

  let sections_to_show = if sections.is_empty() {
    PORTFOLIO_SECTIONS.iter().map(|x| x.to_string()).collect()
  } else {
    sections
  };

  AppConfig {
    sections: sections_to_show,
    speed_factor,
  }
}