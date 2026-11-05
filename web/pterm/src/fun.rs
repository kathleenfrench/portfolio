// toys: neofetch, who, troutsay, and the frames for `top`.
// everything here returns lines of output; fx.rs animates the ones that move
use ansi_term::{Colour, Style};
use rand::prelude::*;

const ACCENT: Colour = Colour::Yellow;

fn dim(s: &str) -> String {
  Colour::Fixed(244).paint(s).to_string()
}

/// a value the page put on `window` (see web/src/main.js): kfSysinfo, kfWho
#[cfg(target_arch = "wasm32")]
pub fn window_json(name: &str) -> Option<serde_json::Value> {
  #[wasm_bindgen::prelude::wasm_bindgen(inline_js = "export function kf_window_json(name) { return JSON.stringify(window[name] === undefined ? null : window[name]); }")]
  extern "C" {
    fn kf_window_json(name: &str) -> String;
  }
  serde_json::from_str(&kf_window_json(name)).ok().filter(|v: &serde_json::Value| !v.is_null())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn window_json(_name: &str) -> Option<serde_json::Value> {
  None
}

fn session_id() -> String {
  window_json("kfSessionId").and_then(|v| v.as_str().map(String::from)).unwrap_or_default()
}

pub fn uptime(secs: u64) -> String {
  let (d, h, m) = (secs / 86400, secs % 86400 / 3600, secs % 3600 / 60);
  let plural = |n: u64, unit: &str| format!("{} {}{}", n, unit, if n == 1 { "" } else { "s" });
  match (d, h) {
    (0, 0) => plural(m, "min"),
    (0, _) => format!("{}, {}", plural(h, "hour"), plural(m, "min")),
    _ => format!("{}, {}", plural(d, "day"), plural(h, "hour")),
  }
}

///// neofetch

const LOGO: &[&str] = &[
  r" _          __ ",
  r"| | __     / _|",
  r"| |/ /    | |_ ",
  r"|   <     |  _|",
  r"|_|\_\    |_|  ",
];

pub fn neofetch(user: &str, cols: usize, rows: usize) -> Vec<String> {
  let info = window_json("kfSysinfo");
  let field = |k: &str| info.as_ref().and_then(|i| i.get(k)).and_then(|v| v.as_str()).map(String::from);
  let up = info.as_ref().and_then(|i| i.get("uptime_secs")).and_then(|v| v.as_u64());

  let title = format!("{}@portfolio", user);
  let mut build = vec![format!("v{}", field("version").unwrap_or_else(|| "0.2.0".into()))];
  build.extend(field("built"));
  build.extend(field("commit"));

  let key = |k: &str| ACCENT.bold().paint(format!("{}:", k)).to_string();
  let mut right = vec![
    ACCENT.bold().paint(title.clone()).to_string(),
    "-".repeat(title.len()),
    format!("{} kathleenfrench.co", key("OS")),
    format!("{} heroku (container)", key("Host")),
    format!("{} actix-web 4 · {}", key("Kernel"), field("rustc").unwrap_or_else(|| "rustc".into())),
    format!("{} {}", key("Uptime"), up.map(uptime).unwrap_or_else(|| "a while".into())),
    format!("{} pterm (rust → wasm)", key("Shell")),
    format!("{} xterm.js 5.3", key("Terminal")),
    format!("{} {}x{}", key("Resolution"), cols, rows),
    format!("{} {}", key("Build"), build.join(" · ")),
    "".into(),
    // the palette swatches, as background-coloured blocks like real neofetch
    [Colour::Black, Colour::Red, Colour::Green, Colour::Yellow, Colour::Blue, Colour::Purple, Colour::Cyan, Colour::White]
      .iter()
      .map(|c| Style::new().on(*c).paint("   ").to_string())
      .collect::<String>(),
  ];

  // side by side when there's room, stacked on phones
  if cols < 64 {
    let mut out: Vec<String> = LOGO.iter().map(|l| ACCENT.paint(*l).to_string()).collect();
    out.push("".into());
    out.append(&mut right);
    return out;
  }
  let height = LOGO.len().max(right.len());
  right.resize(height, String::new());
  (0..height)
    .map(|i| {
      let logo = LOGO.get(i).copied().unwrap_or("               ");
      format!("{}   {}", ACCENT.paint(logo), right[i])
    })
    .collect()
}

///// who

pub fn who() -> Vec<String> {
  let me = session_id();
  let mut sessions: Vec<(String, u64)> = window_json("kfWho")
    .and_then(|v| v.as_array().cloned())
    .unwrap_or_default()
    .iter()
    .filter_map(|s| Some((s.get("id")?.as_str()?.to_string(), s.get("idle_secs")?.as_u64()?)))
    .collect();
  // you first, then the others by how recently they were around
  if !sessions.iter().any(|(id, _)| *id == me) {
    sessions.push((me.clone(), 0));
  }
  sessions.sort_by_key(|(id, idle)| (*id != me, *idle));
  render_who(&sessions, &me)
}

pub fn render_who(sessions: &[(String, u64)], me: &str) -> Vec<String> {
  sessions
    .iter()
    .enumerate()
    .map(|(i, (id, idle))| {
      let when = if *idle < 60 { "active".to_string() } else { format!("{} min idle", idle / 60) };
      let you = if id == me { dim("  (you)") } else { String::new() };
      format!("stranger   pts/{:<4} {:<12}{}", i, when, you)
    })
    .collect()
}

///// troutsay

const TROUT: &[&str] = &[
  r"     \",
  r"      \     |\    \\\\__     o",
  r"       \    | \_/    o \    o",
  r"            > _   (( <_  oo",
  r"            | / \__+___/",
  r"            |/     |/",
];

const TROUT_THOUGHTS: &[&str] = &[
  "is it second dinner yet?",
  "oh i'm sorry did my snoring wake you?",
  "a stranger called me an ozempic frenchie can you believe that",
  "have you tried turning the stranger off and on again",
  "these ears were not made to listen",
];

pub fn troutsay(args: &[String], stdin: &Option<Vec<String>>) -> Vec<String> {
  let text = if !args.is_empty() {
    args.join(" ")
  } else if let Some(input) = stdin.as_ref().filter(|i| !i.is_empty()) {
    input.join(" ")
  } else {
    TROUT_THOUGHTS.choose(&mut thread_rng()).unwrap().to_string()
  };

  let lines = crate::md::wrap(&text, 40);
  let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
  let pad = |l: &str| format!("{}{}", l, " ".repeat(width - l.chars().count()));

  let mut out = vec![format!(" {}", "_".repeat(width + 2))];
  if lines.len() == 1 {
    out.push(format!("< {} >", pad(&lines[0])));
  } else {
    for (i, l) in lines.iter().enumerate() {
      let (open, close) = match i {
        0 => ('/', '\\'),
        _ if i == lines.len() - 1 => ('\\', '/'),
        _ => ('|', '|'),
      };
      out.push(format!("{} {} {}", open, pad(l), close));
    }
  }
  out.push(format!(" {}", "-".repeat(width + 2)));
  out.extend(TROUT.iter().map(|l| l.to_string()));
  out
}

///// top

pub struct Proc {
  pid: u32,
  user: &'static str,
  cmd: &'static str,
  // typical cpu %, and how much it wobbles frame to frame
  cpu: f32,
  jitter: f32,
  mem: f32,
}

pub const PROCS: &[Proc] = &[
  Proc { pid: 1, user: "root", cmd: "init", cpu: 0.0, jitter: 0.1, mem: 0.1 },
  Proc { pid: 23, user: "root", cmd: "actix-web --workers 4", cpu: 2.0, jitter: 1.5, mem: 1.2 },
  Proc { pid: 42, user: "kathleen", cmd: "writing.sh --draft 14", cpu: 12.0, jitter: 6.0, mem: 2.4 },
  Proc { pid: 77, user: "trout", cmd: "beg --until=snack", cpu: 0.3, jitter: 0.3, mem: 0.2 },
  Proc { pid: 81, user: "trout", cmd: "snore --during=my-work-meetings", cpu: 4.0, jitter: 3.0, mem: 0.1 },
  Proc { pid: 88, user: "kathleen", cmd: "procrastinate", cpu: 31.0, jitter: 8.0, mem: 8.0 },
  Proc { pid: 93, user: "kathleen", cmd: "kubectl get pods -w", cpu: 1.5, jitter: 1.0, mem: 0.6 },
  Proc { pid: 104, user: "root", cmd: "the_site.sh", cpu: 0.0, jitter: 0.0, mem: 0.1 },
  Proc { pid: 131, user: "stranger", cmd: "zsh", cpu: 0.4, jitter: 0.4, mem: 0.3 },
];

/// one screen of `top`. `clock` is "HH:MM:SS", `users` how many are on
pub fn top_frame<R: Rng>(rng: &mut R, clock: &str, up_secs: u64, users: usize, width: usize) -> Vec<String> {
  let mut rows: Vec<(f32, &Proc)> = PROCS
    .iter()
    .map(|p| (((p.cpu + rng.gen_range(-p.jitter, p.jitter + 0.001)).max(0.0) * 10.0).round() / 10.0, p))
    .collect();
  rows.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

  let busy: f32 = rows.iter().map(|(c, _)| c).sum();
  let load = |f: f32| format!("{:.2}", busy / 100.0 * f);
  let header = Style::new().reverse().paint(format!("{:<width$}", "  PID USER      %CPU  %MEM  COMMAND", width = width.min(80))).to_string();

  let mut out = vec![
    format!(
      "top - {} up {}, {} user{}, load average: {}, {}, {}",
      clock, uptime(up_secs), users, if users == 1 { "" } else { "s" }, load(1.0), load(0.9), load(0.8),
    ),
    format!("Tasks: {:>3} total,   1 running, {:>3} sleeping", PROCS.len(), PROCS.len() - 1),
    format!("%Cpu(s): {:>5.1} us, {:>5.1} id", busy.min(100.0), (100.0 - busy).max(0.0)),
    "".into(),
    header,
  ];
  out.extend(rows.iter().map(|(cpu, p)| format!("{:>5} {:<9} {:>4.1}  {:>4.1}  {}", p.pid, p.user, cpu, p.mem, p.cmd)));
  out.push("".into());
  out.push(dim("press q to quit"));
  out
}

///// rm -rf /

/// `rm -rf /` (or -fr, -r -f, /*, --no-preserve-root ...)
pub fn is_meltdown(line: &str) -> bool {
  let words: Vec<&str> = line.split_whitespace().collect();
  if words.first() != Some(&"rm") {
    return false;
  }
  let flags: String = words.iter().filter(|w| w.starts_with('-') && !w.starts_with("--")).flat_map(|w| w.chars().skip(1)).collect();
  let recursive = flags.contains('r') || flags.contains('R') || words.contains(&"--recursive");
  let root = words.iter().any(|w| *w == "/" || *w == "/*");
  recursive && root
}

pub const DOOMED: &[&str] = &[
  "/bin/ls", "/bin/cat", "/bin/zsh", "/boot/vmlinuz", "/etc/passwd", "/etc/hosts", "/etc/fstab",
  "/home/kathleen/.ssh/id_ed25519", "/home/kathleen/Documents/novel_draft_FINAL.docx",
  "/home/kathleen/Documents/novel_draft_FINAL_v2.docx", "/home/kathleen/Documents/novel_draft_FINAL_v2_actually.docx",
  "/home/kathleen/Documents/heated_rivalry_fic_ch1.docx",
"/home/stranger/.read_me", "/opt/saudade/event_site/README",
  "/tmp/.scratch", "/usr/bin/git", "/usr/bin/kubectl", "/usr/lib/libtrout.so.1", "/usr/share/dict/words",
  "/var/log/syslog", "/var/lib/heroku/dyno", "/var/www/kathleenfrench.co/index.html",
];

#[cfg(test)]
mod tests {
  use super::*;

  fn plain(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|l| crate::shell::strip_ansi(l)).collect()
  }

  #[test]
  fn troutsay_draws_a_bubble() {
    let out = plain(troutsay(&["hello".into()], &None));
    assert_eq!(out[0], " _______");
    assert_eq!(out[1], "< hello >");
    assert_eq!(out[2], " -------");
    assert!(out[4].contains("\\\\\\\\__"));

    let long = plain(troutsay(&["a much longer message that will need to wrap onto more lines".into()], &None));
    assert!(long[1].starts_with('/') && long[2].starts_with('\\'), "{:?}", long);
    assert_eq!(long[1].len(), long[2].len(), "bubble lines are padded to the same width");
  }

  #[test]
  fn meltdown_detection() {
    for yes in ["rm -rf /", "rm -fr /", "rm -r -f /", "rm -rf /*", "rm -rf --no-preserve-root /", "rm -R /"] {
      assert!(is_meltdown(yes), "{}", yes);
    }
    for no in ["rm -rf ~", "rm /", "rm -rf /tmp", "ls -rf /", "rm -f /"] {
      assert!(!is_meltdown(no), "{}", no);
    }
  }

  #[test]
  fn who_marks_you() {
    let out = plain(render_who(&[("me".into(), 0), ("other".into(), 150)], "me"));
    assert_eq!(out[0].trim_end(), "stranger   pts/0    active        (you)");
    assert_eq!(out[1].trim_end(), "stranger   pts/1    2 min idle");
  }

  #[test]
  fn uptime_reads_like_top() {
    assert_eq!(uptime(59), "0 mins");
    assert_eq!(uptime(3 * 3600 + 12 * 60), "3 hours, 12 mins");
    assert_eq!(uptime(86400 + 3600), "1 day, 1 hour");
  }

  #[test]
  fn top_sorts_by_cpu() {
    let frame = plain(top_frame(&mut thread_rng(), "23:51:04", 3600, 2, 80));
    assert!(frame[0].starts_with("top - 23:51:04 up 1 hour, 0 mins, 2 users"));
    let cpus: Vec<f32> = frame[5..5 + PROCS.len()].iter().map(|l| l.split_whitespace().nth(2).unwrap().parse().unwrap()).collect();
    assert!(cpus.windows(2).all(|w| w[0] >= w[1]), "{:?}", cpus);
  }

  #[test]
  fn neofetch_stacks_on_phones() {
    let wide = plain(neofetch("stranger", 120, 40));
    assert!(wide[0].starts_with(LOGO[0]) && wide[0].ends_with("stranger@portfolio"));
    let narrow = plain(neofetch("stranger", 50, 40));
    assert_eq!(narrow[0], LOGO[0]);
    assert!(narrow.iter().any(|l| l == "Resolution: 50x40"));
  }
}
