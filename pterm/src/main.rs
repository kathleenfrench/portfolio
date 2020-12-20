use yansi::Paint;
use anyhow::Result;

use pterm::app::parse_inputs;
use pterm::{quit, run, PORTFOLIO_SECTIONS, SPEED_FACTOR};

#[async_std::main]
async fn main() -> Result<()> {
    Paint::enable_windows_ascii();

    let cfg = parse_inputs();
    *SPEED_FACTOR.lock().await = cfg.speed_factor;

    if cfg.list_sections_and_quit {
      println!("available sections:");
      for s in PORTFOLIO_SECTIONS {
        println!(" {}", s);
      }

      std::process::exit(0);
    }

    ctrlc::set_handler(quit)?;

    run(cfg).await;

    Ok(())
}