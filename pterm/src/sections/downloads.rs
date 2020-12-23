use crate::app::AppConfig;
use crate::io::{csleep, delayed_print, new_line, print, clear_line, get_terminal_width};
use yansi::Paint;
use rand::prelude::*;
use std::cmp::max;

use crate::content::{TMP_EXTENSIONS_LIST, TMP_SOURCE_LIST};
use crate::utils::gen_file_name_with_extension;

use file_size_opts::FileSizeOpts;
use humansize::{file_size_opts, FileSize};
use humantime::format_duration;
use std::time::Duration;

pub async fn run(cfg: &AppConfig) {
  let mut rng = thread_rng();
  let ext = TMP_EXTENSIONS_LIST.choose(&mut rng).unwrap_or(&".mp3");
  let download_speed = rng.gen_range(10_000_000, 100_000_000);
  let loop_limit = TMP_SOURCE_LIST.len();

  for loop_num in 0..loop_limit {
    let fbytes = rng.gen_range(30_000_000, 300_000_000);
    let sleep_wait_ms = 10;
    let file_name = &format!("/var/tmp/{}", &gen_file_name_with_extension(&mut rng, &TMP_SOURCE_LIST, ext));
    let stats_width = 32;
    let rest_padding = 15;

    if loop_num >= loop_limit {
      print(Paint::green(format!("\r\n{}\r\n", "DONE")).bold().to_string()).await;
      return;
    }

    if get_terminal_width() < stats_width + rest_padding + 7 {
      delayed_print("Terminal too small to display download progress\n", 10).await;
      continue;
    }

    let remaining_width = get_terminal_width() - stats_width;
    let file_name_width = remaining_width / 3;
    let full_progress_bar_size = remaining_width - file_name_width - rest_padding;
    let mut progress_bar = progress_string::BarBuilder::new()
        .total(fbytes as usize)
        .full_char('=')
        .width(full_progress_bar_size)
        .get_bar();

    let mut bytes_downloaded = 0u64;

    loop {
      let download_speed_offset = rng.gen_range(-5_000_000i32, 5_000_000i32);
      let actual_download_speed = max(100_000, download_speed + download_speed_offset) as u64;
      let percent = ((100.0 / fbytes as f64) * bytes_downloaded as f64).min(100.0);

      let bytes_incoming = (actual_download_speed / 1000) * sleep_wait_ms;

      let eta = if bytes_downloaded == 0 {
          Duration::default()
      } else {
          let remaining_secs = (fbytes as i64 - bytes_downloaded as i64).max(0)
              / actual_download_speed as i64;
          Duration::from_secs(remaining_secs as u64)
      };

      clear_line().await;
      progress_bar.replace(bytes_downloaded as usize);
      let size_opts = FileSizeOpts {
          space: false,
          ..file_size_opts::BINARY
      };

      let speed_opts = FileSizeOpts {
          space: false,
          suffix: "/s",
          ..file_size_opts::BINARY
      };

      print(format!(
              "{file_name:<file_name_width$} {percent:>4.0}%{progress_bar} {bytes_downloaded:<10} {download_speed:<12} eta {eta:<10}",
              file_name = file_name.chars().take(file_name_width).collect::<String>(),
              percent = percent,
              progress_bar = progress_bar.to_string(),
              bytes_downloaded = bytes_incoming.file_size(size_opts).unwrap(),
              download_speed = actual_download_speed.file_size(speed_opts).unwrap(),
              eta = format_duration(eta).to_string(),
              file_name_width = file_name_width,
      )).await;

      csleep(sleep_wait_ms).await;

      bytes_downloaded += bytes_incoming;

      if percent >= 100.0 {
          break;
      }

      if cfg.should_quit() {
          return;
      }
    }

    new_line().await;
  }
}