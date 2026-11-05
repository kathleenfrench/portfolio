// a tiny fake filesystem + coreutils, so poking around feels like a real shell.
// everything here returns lines of output (pure, so it's unit tested); `run`
// writes them to the terminal.
//
// the lore: you're `stranger`, logged into kathleen's machine. your home is
// /home/stranger, but the interesting stuff in it is kathleen's and drwx------,
// so you can't open it. /tmp is world readable, which is where the treasure
// hunt's .top_secret lives.
use ansi_term::Colour;
use std::cell::RefCell;
use xterm_js_rs::Terminal;

pub const HOME: &str = "/home/stranger";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
  Dir,
  File,
}

struct Entry {
  path: &'static str,
  kind: Kind,
  // can `stranger` read it
  readable: bool,
  owner: &'static str,
  size: u32,
  date: &'static str,
  contents: fn() -> Vec<String>,
}

fn nothing() -> Vec<String> {
  vec![]
}

fn read_me() -> Vec<String> {
  vec![
    "".into(),
    "“History is the angle at which realities meet.”".into(),
    "― Don DeLillo, End Zone".into(),
    "".into(),
  ]
}

fn top_secret() -> Vec<String> {
  vec!["nice work :)".into(), "run 'goto l00t' for a surprise...".into()]
}

macro_rules! dir {
  ($path:expr, $readable:expr, $owner:expr, $date:expr) => {
    Entry { path: $path, kind: Kind::Dir, readable: $readable, owner: $owner, size: 4096, date: $date, contents: nothing }
  };
}

macro_rules! file {
  ($path:expr, $readable:expr, $owner:expr, $size:expr, $date:expr, $contents:expr) => {
    Entry { path: $path, kind: Kind::File, readable: $readable, owner: $owner, size: $size, date: $date, contents: $contents }
  };
}

const FS: &[Entry] = &[
  dir!("/", true, "root", "Jan  5  2026"),
  dir!("/bin", false, "root", "Jan  5  2026"),
  dir!("/etc", false, "root", "Jan  5  2026"),
  dir!("/home", true, "root", "Jan  5  2026"),
  dir!("/root", false, "root", "Jan  5  2026"),
  dir!("/tmp", true, "root", "Nov  4 03:14"),
  dir!("/usr", false, "root", "Jan  5  2026"),
  dir!("/var", false, "root", "Jan  5  2026"),
  dir!("/home/kathleen", false, "kathleen", "Nov  4 09:41"),
  dir!("/home/stranger", true, "stranger", "Nov  4 09:41"),
  dir!("/home/stranger/Documents", false, "kathleen", "Oct 13 21:06"),
  dir!("/home/stranger/Downloads", false, "kathleen", "Nov  2 18:22"),
  dir!("/home/stranger/Pictures", false, "kathleen", "Oct 19 00:49"),
  dir!("/home/stranger/.ssh", false, "kathleen", "Jan  5  2026"),
  file!("/home/stranger/.bashrc", false, "kathleen", 3771, "Feb 14  2025", nothing),
  file!("/home/stranger/.vimrc", false, "kathleen", 612, "Sep  9  2024", nothing),
  file!("/home/stranger/.read_me", true, "kathleen", 76, "Nov 15  2020", read_me),
  file!("/tmp/.top_secret", true, "root", 49, "Nov 15  2020", top_secret),
];

thread_local! {
  static CWD: RefCell<String> = RefCell::new(HOME.to_string());
}

fn cwd() -> String {
  CWD.with(|c| c.borrow().clone())
}

fn lookup(path: &str) -> Option<&'static Entry> {
  FS.iter().find(|e| e.path == path)
}

fn parent(path: &str) -> &str {
  match path.rfind('/') {
    Some(0) => "/",
    Some(i) => &path[..i],
    None => "/",
  }
}

fn base(path: &str) -> &str {
  if path == "/" { "/" } else { path.rsplit('/').next().unwrap_or(path) }
}

fn children(dir: &str) -> Vec<&'static Entry> {
  FS.iter().filter(|e| e.path != "/" && parent(e.path) == dir).collect()
}

/// resolve `arg` against `cwd` into an absolute path: handles ~, ., .., and
/// trailing slashes. doesn't care whether the path exists
pub fn resolve(cwd: &str, arg: &str) -> String {
  let start = if arg == "~" || arg.starts_with("~/") {
    HOME.to_string() + &arg[1..]
  } else if arg.starts_with('/') {
    arg.to_string()
  } else {
    format!("{}/{}", cwd, arg)
  };

  let mut parts: Vec<&str> = vec![];
  for p in start.split('/') {
    match p {
      "" | "." => {}
      ".." => {
        parts.pop();
      }
      p => parts.push(p),
    }
  }
  format!("/{}", parts.join("/"))
}

fn err(line: String) -> String {
  Colour::Red.paint(line).to_string()
}

/// split a command line into words, honoring simple '...' and "..." quoting
pub fn split_args(line: &str) -> Vec<String> {
  let mut args = vec![];
  let mut cur = String::new();
  let mut quote: Option<char> = None;
  let mut has_word = false;

  for c in line.chars() {
    match (quote, c) {
      (Some(q), c) if c == q => quote = None,
      (Some(_), c) => cur.push(c),
      (None, '\'' | '"') => {
        quote = Some(c);
        has_word = true;
      }
      (None, c) if c.is_whitespace() => {
        if has_word || !cur.is_empty() {
          args.push(std::mem::take(&mut cur));
          has_word = false;
        }
      }
      (None, c) => cur.push(c),
    }
  }
  if has_word || !cur.is_empty() {
    args.push(cur);
  }
  args
}

///// commands

pub fn ls(args: &[String]) -> Vec<String> {
  let mut all = false;
  let mut almost_all = false;
  let mut long = false;
  let mut human = false;
  let mut paths: Vec<&str> = vec![];

  for a in args {
    if a == "--all" {
      all = true;
    } else if a.starts_with("--") {
      return vec![err(format!("ls: unrecognized option '{}'", a)), "Try 'ls --help' for more information.".into()];
    } else if a.starts_with('-') && a.len() > 1 {
      for f in a.chars().skip(1) {
        match f {
          'a' => all = true,
          'A' => almost_all = true,
          'l' => long = true,
          'h' => human = true,
          '1' | 'r' | 't' | 'S' | 'G' | 'F' => {}
          other => {
            return vec![err(format!("ls: invalid option -- '{}'", other)), "Try 'ls --help' for more information.".into()];
          }
        }
      }
    } else {
      paths.push(a);
    }
  }

  let show_hidden = all || almost_all;
  let cwd = cwd();
  if paths.is_empty() {
    paths.push(".");
  }

  let mut out = vec![];
  let multiple = paths.len() > 1;

  for (i, arg) in paths.iter().enumerate() {
    let path = resolve(&cwd, arg);
    let entry = match lookup(&path) {
      Some(e) => e,
      None => {
        out.push(err(format!("ls: cannot access '{}': No such file or directory", arg)));
        continue;
      }
    };

    if entry.kind == Kind::File {
      out.push(if long { long_line(entry, arg, human) } else { arg.to_string() });
      continue;
    }

    if !entry.readable {
      out.push(err(format!("ls: cannot open directory '{}': Permission denied", arg)));
      continue;
    }

    if multiple {
      if i > 0 {
        out.push("".into());
      }
      out.push(format!("{}:", arg));
    }

    // sorted like `ls` with a normal locale: case-insensitive, ignoring the
    // leading dot. /tmp's .top_secret always shows: it's the treasure hunt
    // clue, and the hint is a plain `ls /tmp`
    let mut kids: Vec<&Entry> = children(&path)
      .into_iter()
      .filter(|e| show_hidden || !base(e.path).starts_with('.') || path == "/tmp")
      .collect();
    kids.sort_by_key(|e| base(e.path).trim_start_matches('.').to_lowercase());

    let mut names: Vec<(String, Option<&Entry>)> = vec![];
    if all {
      names.push((".".into(), Some(entry)));
      names.push(("..".into(), lookup(parent(&path))));
    }
    names.extend(kids.iter().map(|e| (base(e.path).to_string(), Some(*e))));

    if long {
      let blocks: u32 = names.iter().filter_map(|(_, e)| *e).map(|e| (e.size + 4095) / 4096 * 4).sum();
      out.push(format!("total {}", blocks));
      for (name, e) in &names {
        if let Some(e) = e {
          out.push(long_line(e, name, human));
        }
      }
    } else if !names.is_empty() {
      out.push(names.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join("  "));
    }
  }

  out
}

fn long_line(e: &Entry, name: &str, human: bool) -> String {
  let mode = match (e.kind, e.readable) {
    (Kind::Dir, true) => "drwxr-xr-x",
    (Kind::Dir, false) => "drwx------",
    (Kind::File, true) => "-rw-r--r--",
    (Kind::File, false) => "-rw-------",
  };
  let links = if e.kind == Kind::Dir { 2 } else { 1 };
  let size = if human && e.size >= 1024 { format!("{:.1}K", e.size as f32 / 1024.0) } else { e.size.to_string() };
  format!("{} {:>2} {:<8} {:<8} {:>5} {} {}", mode, links, e.owner, e.owner, size, e.date, name)
}

pub fn cat(args: &[String]) -> Vec<String> {
  if args.is_empty() {
    return vec!["usage: cat [file ...]".into()];
  }

  let cwd = cwd();
  let mut out = vec![];
  for arg in args {
    match lookup(&resolve(&cwd, arg)) {
      None => out.push(err(format!("cat: {}: No such file or directory", arg))),
      Some(e) if e.kind == Kind::Dir => out.push(err(format!("cat: {}: Is a directory", arg))),
      Some(e) if !e.readable => out.push(err(format!("cat: {}: Permission denied", arg))),
      Some(e) => out.extend((e.contents)()),
    }
  }
  out
}

pub fn cd(args: &[String]) -> Vec<String> {
  let target = match args.first() {
    None => HOME.to_string(),
    Some(a) => resolve(&cwd(), a),
  };
  let shown = args.first().map(|s| s.as_str()).unwrap_or("~");

  match lookup(&target) {
    None => vec![err(format!("cd: no such file or directory: {}", shown))],
    Some(e) if e.kind == Kind::File => vec![err(format!("cd: not a directory: {}", shown))],
    Some(e) if !e.readable => vec![err(format!("cd: permission denied: {}", shown))],
    Some(_) => {
      CWD.with(|c| *c.borrow_mut() = target);
      vec![]
    }
  }
}

pub fn echo(args: &[String]) -> Vec<String> {
  let words: Vec<String> = args.iter().map(|a| expand(a)).collect();
  vec![words.join(" ")]
}

// $VAR expansion for the variables `env` shows
fn expand(word: &str) -> String {
  let mut out = word.to_string();
  for (k, v) in env_vars() {
    out = out.replace(&format!("${{{}}}", k), &v).replace(&format!("${}", k), &v);
  }
  out
}

pub fn env_vars() -> Vec<(&'static str, String)> {
  vec![
    ("EDITOR", "vim".into()),
    ("HOME", HOME.into()),
    ("LANG", "en_US.UTF-8".into()),
    ("LOGNAME", "stranger".into()),
    ("PATH", "/usr/local/bin:/usr/bin:/bin".into()),
    ("PWD", cwd()),
    ("SHELL", "/bin/zsh".into()),
    ("TMPDIR", "/tmp".into()),
    ("USER", "stranger".into()),
  ]
}

const BINARIES: &[&str] = &[
  "cat", "curl", "date", "git", "hostname", "id", "less", "ls", "man", "mkdir", "nano", "ps", "python3", "rm",
  "ssh", "touch", "uname", "vim", "wget", "whoami",
];
const BUILTINS: &[&str] = &["cd", "echo", "exit", "history", "pwd", "which"];

pub fn which(args: &[String]) -> Vec<String> {
  args
    .iter()
    .map(|a| {
      if BUILTINS.contains(&a.as_str()) {
        format!("{}: shell built-in command", a)
      } else if BINARIES.contains(&a.as_str()) {
        format!("/usr/bin/{}", a)
      } else {
        format!("{} not found", a)
      }
    })
    .collect()
}

const GIT_REPO_COMMANDS: &[&str] = &[
  "add", "bisect", "blame", "branch", "checkout", "cherry-pick", "commit", "diff", "fetch", "log", "merge",
  "mv", "pull", "push", "rebase", "remote", "reset", "restore", "rm", "show", "stash", "status", "switch", "tag",
];

pub fn git(args: &[String]) -> Vec<String> {
  let sub = args.first().map(|s| s.as_str());
  match sub {
    None | Some("help") | Some("--help") | Some("-h") => vec![
      "usage: git [--version] [--help] <command> [<args>]".into(),
      "".into(),
      "start a working area:  clone, init".into(),
      "work on changes:       add, mv, restore, rm".into(),
      "examine history:       bisect, diff, log, show, status".into(),
      "grow and tweak:        branch, commit, merge, rebase, reset, switch, tag".into(),
      "collaborate:           fetch, pull, push".into(),
    ],
    Some("--version") | Some("version") => vec!["git version 2.47.0".into()],
    Some("init") => vec![err("fatal: cannot mkdir .git: Permission denied".into())],
    Some("clone") => match args.iter().skip(1).find(|a| !a.starts_with('-')) {
      None => vec![err("fatal: You must specify a repository to clone.".into())],
      Some(url) => {
        let name = url.trim_end_matches('/').trim_end_matches(".git").rsplit(['/', ':']).next().unwrap_or("repo").to_string();
        vec![
          format!("Cloning into '{}'...", name),
          err(format!("fatal: could not create work tree dir '{}': Permission denied", name)),
        ]
      }
    },
    Some(cmd) if GIT_REPO_COMMANDS.contains(&cmd) => {
      vec![err("fatal: not a git repository (or any of the parent directories): .git".into())]
    }
    Some(cmd) => vec![err(format!("git: '{}' is not a git command. See 'git --help'.", cmd))],
  }
}

/// the rest of the coreutils-ish commands, by name. `None` = not a command we know
pub fn simple(cmd: &str, args: &[String]) -> Option<Vec<String>> {
  let first = args.first().map(|s| s.as_str()).unwrap_or("");
  let each = |msg: &dyn Fn(&str) -> String, missing: &str| -> Vec<String> {
    if args.is_empty() { vec![err(missing.to_string())] } else { args.iter().map(|a| err(msg(a))).collect() }
  };

  Some(match cmd {
    "pwd" => vec![cwd()],
    "rm" | "rmdir" => each(&|a| format!("{}: cannot remove '{}': Permission denied", cmd, a), &format!("{}: missing operand", cmd)),
    "touch" => each(&|a| format!("touch: cannot touch '{}': Permission denied", a), "touch: missing file operand"),
    "mkdir" => each(&|a| format!("mkdir: cannot create directory '{}': Permission denied", a), "mkdir: missing operand"),
    "mv" | "cp" | "ln" => vec![err(format!("{}: permission denied", cmd))],
    "vim" | "vi" | "nvim" | "nano" | "emacs" | "less" | "more" | "python" | "python3" | "node" | "kill" | "apt" | "brew" => {
      vec![err(format!("zsh: permission denied: {}", cmd))]
    }
    "ps" => vec![
      "  PID TTY          TIME CMD".into(),
      "    1 pts/0    00:00:00 zsh".into(),
      "   42 pts/0    00:00:00 ps".into(),
    ],
    "id" => vec!["uid=1001(stranger) gid=1001(stranger) groups=1001(stranger)".into()],
    "hostname" => vec!["portfolio".into()],
    "uname" if args.iter().any(|a| a == "-a") => vec!["Linux portfolio 6.8.0-kf #1 SMP x86_64 GNU/Linux".into()],
    "uname" => vec!["Linux".into()],
    "man" if args.is_empty() => vec!["What manual page do you want?".into()],
    "man" => vec![format!("No manual entry for {}", first)],
    "git" => git(args),
    "ssh" if args.is_empty() => vec!["usage: ssh destination [command]".into()],
    "ssh" => vec![err(format!("ssh: connect to host {} port 22: Connection refused", args.last().unwrap()))],
    "curl" | "wget" if args.is_empty() => vec![format!("{}: try '{} --help' for more information", cmd, cmd)],
    "curl" | "wget" => vec![err(format!("{}: (6) Could not resolve host: {}", cmd, args.last().unwrap()))],
    "exit" | "logout" => vec!["logout".into(), Colour::Fixed(244).paint("...nice try. there's no leaving :)").to_string()],
    "which" => which(args),
    "echo" => echo(args),
    "cat" => cat(args),
    "cd" => cd(args),
    "ls" => ls(args),
    _ => return None,
  })
}

/// run a command line. returns false if it isn't a command we know
pub fn run(term: &Terminal, line: &str) -> bool {
  let words = split_args(line);
  let (cmd, args) = match words.split_first() {
    Some((c, a)) => (c.as_str(), a),
    None => return true,
  };

  let out = match cmd {
    "date" => vec![now()],
    _ => match simple(cmd, args) {
      Some(out) => out,
      None => return false,
    },
  };

  for l in out {
    term.writeln(&l);
  }
  true
}

pub fn command_not_found(term: &Terminal, line: &str) {
  let cmd = split_args(line).into_iter().next().unwrap_or_default();
  term.writeln(&err(format!("zsh: command not found: {}", cmd)));
}

// like `date`: `Wed Nov  4 21:46:59 2026`
fn now() -> String {
  let now = js_sys::Date::new_0();
  let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  format!(
    "{} {} {:>2} {:02}:{:02}:{:02} {}",
    days[now.get_day() as usize], months[now.get_month() as usize], now.get_date(),
    now.get_hours(), now.get_minutes(), now.get_seconds(), now.get_full_year(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  fn args(s: &str) -> Vec<String> {
    split_args(s)
  }

  // strip ansi color codes so tests read like terminal output
  fn plain(lines: Vec<String>) -> Vec<String> {
    lines
      .into_iter()
      .map(|l| {
        let mut out = String::new();
        let mut esc = false;
        for c in l.chars() {
          match (esc, c) {
            (false, '\x1b') => esc = true,
            (true, 'm') => esc = false,
            (true, _) => {}
            (false, c) => out.push(c),
          }
        }
        out
      })
      .collect()
  }

  fn reset() {
    CWD.with(|c| *c.borrow_mut() = HOME.to_string());
  }

  #[test]
  fn resolves_paths() {
    assert_eq!(resolve(HOME, ".ssh"), "/home/stranger/.ssh");
    assert_eq!(resolve(HOME, "./.ssh/"), "/home/stranger/.ssh");
    assert_eq!(resolve(HOME, "~/.read_me"), "/home/stranger/.read_me");
    assert_eq!(resolve(HOME, "../.."), "/");
    assert_eq!(resolve("/tmp", "../home/stranger"), "/home/stranger");
    assert_eq!(resolve(HOME, "/tmp/.top_secret"), "/tmp/.top_secret");
  }

  #[test]
  fn ls_home() {
    reset();
    for home in ["", ".", "~", "/home/stranger", "/home/stranger/"] {
      assert_eq!(plain(ls(&args(home))), vec!["Documents  Downloads  Pictures"], "ls {}", home);
    }
    assert_eq!(plain(ls(&args("/home"))), vec!["kathleen  stranger"]);
    assert_eq!(plain(ls(&args("-A"))), vec![".bashrc  Documents  Downloads  Pictures  .read_me  .ssh  .vimrc"]);
    let long = plain(ls(&args("-lah")));
    assert!(long[0].starts_with("total "));
    assert!(long.iter().any(|l| l.starts_with("drwx------  2 kathleen kathleen  4.0K") && l.ends_with(" .ssh")));
  }

  #[test]
  fn ls_private_dirs_are_denied_with_or_without_flags() {
    reset();
    for cmd in [".ssh", "-la .ssh", "-lah ~/.ssh/", "Documents", "/root", "/home/kathleen"] {
      let out = plain(ls(&args(cmd)));
      assert_eq!(out.len(), 1, "{}", cmd);
      assert!(out[0].ends_with("Permission denied"), "{}: {:?}", cmd, out);
    }
  }

  #[test]
  fn ls_errors() {
    reset();
    assert_eq!(plain(ls(&args("nope"))), vec!["ls: cannot access 'nope': No such file or directory"]);
    assert_eq!(plain(ls(&args("-z")))[0], "ls: invalid option -- 'z'");
    assert_eq!(plain(ls(&args(".bashrc"))), vec![".bashrc"]);
  }

  #[test]
  fn treasure_hunt_still_works() {
    reset();
    assert_eq!(plain(ls(&args("/tmp"))), vec![".top_secret"]);
    assert_eq!(plain(cat(&args("/tmp/.top_secret")))[1], "run 'goto l00t' for a surprise...");
    assert!(plain(cat(&args(".read_me"))).iter().any(|l| l.contains("History is the angle")));
  }

  #[test]
  fn cat_errors() {
    reset();
    assert_eq!(plain(cat(&args(".ssh"))), vec!["cat: .ssh: Is a directory"]);
    assert_eq!(plain(cat(&args(".bashrc"))), vec!["cat: .bashrc: Permission denied"]);
    assert_eq!(plain(cat(&args("nope"))), vec!["cat: nope: No such file or directory"]);
  }

  #[test]
  fn cd_and_pwd() {
    reset();
    assert_eq!(plain(cd(&args("Documents"))), vec!["cd: permission denied: Documents"]);
    assert_eq!(plain(cd(&args("nope"))), vec!["cd: no such file or directory: nope"]);
    assert_eq!(plain(cd(&args(".bashrc"))), vec!["cd: not a directory: .bashrc"]);
    assert!(cd(&args("/tmp")).is_empty());
    assert_eq!(simple("pwd", &[]).unwrap(), vec!["/tmp"]);
    assert_eq!(plain(ls(&args(""))), vec![".top_secret"]);
    assert!(cd(&args("")).is_empty());
    assert_eq!(simple("pwd", &[]).unwrap(), vec![HOME]);
  }

  #[test]
  fn echo_and_which() {
    reset();
    assert_eq!(echo(&args("hi $USER, welcome to '$HOME'")), vec!["hi stranger, welcome to /home/stranger"]);
    assert_eq!(which(&args("ls cd nope")), vec!["/usr/bin/ls", "cd: shell built-in command", "nope not found"]);
  }

  #[test]
  fn git_answers_per_subcommand() {
    assert_eq!(plain(git(&args("init"))), vec!["fatal: cannot mkdir .git: Permission denied"]);
    assert_eq!(
      plain(git(&args("clone https://github.com/kathleenfrench/portfolio.git"))),
      vec!["Cloning into 'portfolio'...", "fatal: could not create work tree dir 'portfolio': Permission denied"],
    );
    assert_eq!(plain(git(&args("status"))), vec!["fatal: not a git repository (or any of the parent directories): .git"]);
    assert_eq!(plain(git(&args("frobnicate"))), vec!["git: 'frobnicate' is not a git command. See 'git --help'."]);
    assert!(plain(git(&args("")))[0].starts_with("usage: git"));
    assert_eq!(git(&args("--version")), vec!["git version 2.47.0"]);
  }

  #[test]
  fn unknown_commands_fall_through() {
    assert!(simple("frobnicate", &[]).is_none());
  }
}
