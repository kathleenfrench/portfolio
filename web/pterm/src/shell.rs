// a tiny fake filesystem + coreutils, so poking around feels like a real shell.
// commands return lines of output (pure, so they're unit tested); `run` writes
// them to the terminal.
//
// the lore: you're `stranger`, logged into kathleen's machine. your home is
// /home/stranger, but the interesting stuff is kathleen's and drwx------, so
// you can't open it. /tmp is world readable, which is where the hunt starts.
//
// two puzzles live in here (`hint` walks you through the first):
//   the hunt: .read_me -> history -> /tmp/.scratch -> su kathleen ->
//             ~/treasure.txt -> goto l00t
//   the ctf:  (as kathleen) ~/.bash_history -> crontab -l -> the_entertainment.sh
//             (gpg) -> ~/.ssh/vault_key | base64 -d | xxd -r -p (a riddle) ->
//             motel splendido -> gpg -d .vault -> goto kefahuchi
use ansi_term::Colour;
use std::cell::RefCell;
use xterm_js_rs::Terminal;

pub const STRANGER_HOME: &str = "/home/stranger";
pub const KATHLEEN_HOME: &str = "/home/kathleen";
// what trout is to kathleen (also: wallace's middle name + fail)
const PASSWORD: &str = "fosterfail";
// the vault's gpg passphrase (matching ignores case, spaces and punctuation)
const PASSPHRASE: &str = "motel splendido";
// what ~/.ssh/vault_key decodes to (base64, then hex): a riddle whose answer is
// the passphrase. the line is from m. john harrison's light, about motel splendido
const VAULT_RIDDLE: &str = "pw: where they went out one morning with their lives all washed up,
and returned in the evening corporate CEOs heavy with patents.
(lowercase, two words)";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
  Dir,
  File,
}

struct Entry {
  path: &'static str,
  kind: Kind,
  // readable by everyone. the owner can always read it
  readable: bool,
  owner: &'static str,
  size: u32,
  date: &'static str,
  contents: fn() -> Vec<String>,
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
  dir!("/opt", true, "root", "Jan  5  2026"),
  dir!("/root", false, "root", "Jan  5  2026"),
  dir!("/tmp", true, "root", "Nov  4 03:14"),
  dir!("/usr", false, "root", "Jan  5  2026"),
  dir!("/var", false, "root", "Jan  5  2026"),
  // the stranger's home: the good stuff is kathleen's
  dir!("/home/stranger", true, "stranger", "Nov  4 09:41"),
  dir!("/home/stranger/Documents", false, "kathleen", "Oct 13 21:06"),
  dir!("/home/stranger/Downloads", false, "kathleen", "Nov  2 18:22"),
  dir!("/home/stranger/Pictures", false, "kathleen", "Oct 19 00:49"),
  dir!("/home/stranger/.ssh", false, "kathleen", "Jan  5  2026"),
  file!("/home/stranger/.bashrc", false, "kathleen", 3771, "Feb 14  2025", nothing),
  file!("/home/stranger/.vimrc", false, "kathleen", 612, "Sep  9  2024", nothing),
  file!("/home/stranger/.read_me", true, "kathleen", 76, "Nov 15  2020", read_me),
  file!("/tmp/.scratch", true, "kathleen", 412, "Nov  4 22:31", scratch),
  // kathleen's home, once you are her
  dir!("/home/kathleen", false, "kathleen", "Nov  4 09:41"),
  file!("/home/kathleen/treasure.txt", false, "kathleen", 96, "Nov  4 21:12", treasure_txt),
  file!("/home/kathleen/.bash_history", false, "kathleen", 140, "Nov  4 03:13", kathleen_history),
  dir!("/home/kathleen/.ssh", false, "kathleen", "Jan  5  2026"),
  file!("/home/kathleen/.ssh/id_ed25519", false, "kathleen", 411, "Jan  5  2026", trout_key),
  file!("/home/kathleen/.ssh/vault_key", false, "kathleen", 73, "Nov  4 03:12", vault_key),
  dir!("/home/kathleen/Documents", false, "kathleen", "Oct 13 21:06"),
  file!("/home/kathleen/Documents/reading_list.txt", false, "kathleen", 233, "Nov  4 20:58", reading_list),
  // the ctf
  dir!("/opt/saudade", false, "kathleen", "Nov  4 03:10"),
  dir!("/opt/saudade/event_site", false, "kathleen", "Nov  4 03:11"),
  file!("/opt/saudade/event_site/README", false, "kathleen", 140, "Nov  4 03:11", event_site_readme),
  file!("/opt/saudade/event_site/the_entertainment.sh", false, "kathleen", 214, "Nov  4 03:11", the_entertainment),
  file!("/opt/saudade/event_site/.vault", false, "kathleen", 304, "Nov  4 03:12", vault),
];

///// file contents

fn nothing() -> Vec<String> {
  vec![]
}

fn lines(s: &str) -> Vec<String> {
  s.lines().map(String::from).collect()
}

fn read_me() -> Vec<String> {
  lines("\n“History is the angle at which realities meet.”\n― Don DeLillo, End Zone\n")
}

// kathleen's scratch file: notes to self, and the hunt's password clue. the 🐟
// is trout, and her origin story (per the about page) is that she's a foster fail
fn scratch() -> Vec<String> {
  lines(
    "# scratch

## hardening todo
- [x] disable root login over ssh
- [x] fail2ban on sshd
- [ ] rotate ssh keys
- [ ] audit the crontab
- [ ] stop leaving passwords in /tmp

## what can this service account actually do?
    kubectl auth can-i --list --as=system:serviceaccount:<namespace>:<sa>

## login
pw = 🐟 origin story (one word, no spaces)",
  )
}

fn treasure_txt() -> Vec<String> {
  lines("you made it, kathleen. or whoever you really are.\n\nthe treasure is this way: goto l00t")
}

fn kathleen_history() -> Vec<String> {
  lines(
    "    1  cd /opt/saudade/event_site\n    2  ls -a\n    3  vim README\n    4  # watchman, what of the night?\n    5  crontab -e\n    6  cat ~/.ssh/vault_key\n    7  clear",
  )
}

fn trout_key() -> Vec<String> {
  lines("-----BEGIN OPENSSH PRIVATE KEY-----\n\n          ><(((º>   (it's a trout)\n\n-----END OPENSSH PRIVATE KEY-----")
}

fn vault_key() -> Vec<String> {
  // two layers: the riddle as hex, then that as base64
  vec![b64_encode(hex(VAULT_RIDDLE.as_bytes()).as_bytes())]
}

fn hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn reading_list() -> Vec<String> {
  lines(
    "* nightwood — djuna barnes\n* infinite jest — david foster wallace\n* a naked singularity — sergio de la pava\n* end zone — don delillo\n* light, nova swing, empty space — m. john harrison",
  )
}

fn event_site_readme() -> Vec<String> {
  lines(
    "# EVENT SITE

> Then he heard a voice say: 'It was amazing to them to discover they had always been in the garden without understanding it', and knew for certain that the inside and the outside of everything are always a single, continuous medium. In that moment he believed he could go anywhere.

  — M. John Harrison, Light",
  )
}

fn the_entertainment() -> Vec<String> {
  lines(
    "#!/bin/sh
# do not watch.
# runs nightly (see crontab). checks the vault still opens.
# the passphrase never leaves root.
gpg --batch --passphrase-file /root/.vault_pass --decrypt /opt/saudade/event_site/.vault > /dev/null",
  )
}

// looks like a gpg-encrypted message; `gpg -d` with the passphrase opens it
fn vault() -> Vec<String> {
  lines(
    "-----BEGIN PGP MESSAGE-----

jA0ECQMCvaE1Ul5/lwU6YwGm3IqW0PCesfYSdWDgB3L73v+uKdyMeLQ91dfQ2x3H
SbBKNhz/xa5RjID8f9KGx35iiHP6UC69ar82+ieaOK0J9fPliFC4foQD0IJZfmFv
MT/TqNnO/IXKxwOGv3WIOI1Pw6ntx2+jChqBG1f8byO+5bj/ieIB2f3GQpzqXxAq
FNaz417oT6D9aT3D
=kf0x
-----END PGP MESSAGE-----",
  )
}

fn vault_plaintext() -> Vec<String> {
  lines("you crossed it.\ngoto kefahuchi")
}

///// state: who you are, where you are, how far you've gotten

#[derive(Clone, Copy, PartialEq)]
enum Prompt {
  Su,
  // decrypting the vault
  Gpg,
}

#[derive(Default)]
struct State {
  cwd: Option<String>,
  kathleen: bool,
  // a hidden-input prompt is waiting for the next line (su's password, gpg's passphrase)
  awaiting: Option<Prompt>,
  // milestones reached (see STEPS) and how many hints shown for each
  progress: Vec<&'static str>,
  hints_shown: Vec<(&'static str, usize)>,
  // what's been typed this session, for `history` and the up arrow
  typed: Vec<String>,
}

thread_local! {
  static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn user() -> &'static str {
  STATE.with(|s| if s.borrow().kathleen { "kathleen" } else { "stranger" })
}

pub fn home() -> &'static str {
  if user() == "kathleen" { KATHLEEN_HOME } else { STRANGER_HOME }
}

/// the current directory the way a prompt shows it: home is `~`
pub fn prompt_dir() -> String {
  let cwd = cwd();
  match cwd.strip_prefix(home()) {
    Some("") => "~".into(),
    Some(rest) if rest.starts_with('/') => format!("~{}", rest),
    _ => cwd,
  }
}

fn cwd() -> String {
  STATE.with(|s| s.borrow().cwd.clone()).unwrap_or_else(|| home().to_string())
}

fn set_cwd(path: String) {
  STATE.with(|s| s.borrow_mut().cwd = Some(path));
}

fn can_read(e: &Entry) -> bool {
  e.readable || e.owner == user()
}

pub fn is_awaiting_password() -> bool {
  STATE.with(|s| s.borrow().awaiting.is_some())
}

/// what to print for the hidden-input prompt
pub fn password_prompt() -> &'static str {
  match STATE.with(|s| s.borrow().awaiting) {
    Some(Prompt::Gpg) => "Enter passphrase: ",
    _ => "Password: ",
  }
}

fn await_input(prompt: Prompt) {
  STATE.with(|s| s.borrow_mut().awaiting = Some(prompt));
}

fn reached(milestone: &str) -> bool {
  STATE.with(|s| s.borrow().progress.iter().any(|m| *m == milestone))
}

fn mark(milestone: &'static str) {
  if reached(milestone) {
    return;
  }
  STATE.with(|s| s.borrow_mut().progress.push(milestone));
  storage::milestone(milestone);
}

/// restore progress saved in the browser (no-op outside the browser)
pub fn load() {
  for m in storage::load_progress() {
    let known = STEPS.iter().map(|s| s.0).chain(CTF_STEPS.iter().copied()).find(|k| *k == m);
    if let Some(k) = known {
      STATE.with(|s| s.borrow_mut().progress.push(k));
    }
  }
}

///// paths

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
    home().to_string() + &arg[1..]
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

fn dim(line: &str) -> String {
  Colour::Fixed(244).paint(line).to_string()
}

/// split a command line into words like a shell: '...' and "..." quoting,
/// $VAR / ${VAR} expansion, and $(command) substitution (none of it inside
/// single quotes)
pub fn split_args(line: &str) -> Vec<String> {
  let mut args = vec![];
  let mut cur = String::new();
  let mut quote: Option<char> = None;
  let mut has_word = false;
  let mut chars = line.chars().peekable();

  while let Some(c) = chars.next() {
    match (quote, c) {
      (Some(q), c) if c == q => quote = None,
      // $(command): run it, splice its output in. unquoted, it splits into words
      (q, '$') if q != Some('\'') && chars.peek() == Some(&'(') => {
        chars.next();
        let mut inner = String::new();
        let mut depth = 1;
        for c in chars.by_ref() {
          match c {
            '(' => depth += 1,
            ')' => {
              depth -= 1;
              if depth == 0 {
                break;
              }
            }
            _ => {}
          }
          inner.push(c);
        }
        let output = run_pipeline(&inner, &now).unwrap_or_default();
        let text = output.iter().map(|l| strip_ansi(l)).collect::<Vec<_>>().join(" ");
        if q.is_some() {
          cur.push_str(&text);
        } else {
          for (i, word) in text.split_whitespace().enumerate() {
            if i > 0 {
              args.push(std::mem::take(&mut cur));
            }
            cur.push_str(word);
          }
        }
        has_word = true;
      }
      (q, '$') if q != Some('\'') => {
        let braced = chars.peek() == Some(&'{');
        if braced {
          chars.next();
        }
        let mut name = String::new();
        while let Some(&n) = chars.peek() {
          if n.is_ascii_alphanumeric() || n == '_' {
            name.push(n);
            chars.next();
          } else {
            break;
          }
        }
        if braced && chars.peek() == Some(&'}') {
          chars.next();
        }
        if name.is_empty() {
          cur.push('$');
        } else {
          cur.push_str(&env_value(&name));
        }
        has_word = true;
      }
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

// an environment variable's value, or "" like a real shell
fn env_value(name: &str) -> String {
  env_vars().into_iter().find(|(k, _)| *k == name).map(|(_, v)| v).unwrap_or_default()
}

/// split on `|` outside of quotes
fn split_pipeline(line: &str) -> Vec<String> {
  let mut stages = vec![];
  let mut cur = String::new();
  let mut quote: Option<char> = None;
  // inside $( ... ), where a | belongs to the inner command
  let mut depth: u32 = 0;
  for c in line.chars() {
    match (quote, c) {
      (Some(q), c) if c == q => {
        quote = None;
        cur.push(c);
      }
      (None, '\'' | '"') => {
        quote = Some(c);
        cur.push(c);
      }
      (None, '|') if depth == 0 => stages.push(std::mem::take(&mut cur)),
      (None, '(') => {
        depth += 1;
        cur.push('(');
      }
      (None, ')') => {
        depth = depth.saturating_sub(1);
        cur.push(')');
      }
      (_, c) => cur.push(c),
    }
  }
  stages.push(cur);
  stages
}

fn strip_ansi(s: &str) -> String {
  let mut out = String::new();
  let mut esc = false;
  for c in s.chars() {
    match (esc, c) {
      (false, '\x1b') => esc = true,
      (true, 'm') => esc = false,
      (true, _) => {}
      (false, c) => out.push(c),
    }
  }
  out
}

// milestones for reading particular files
fn on_read(path: &str) {
  match path {
    "/home/stranger/.read_me" => mark("read_me"),
    "/tmp/.scratch" => mark("scratch"),
    "/home/kathleen/treasure.txt" => mark("treasure"),
    "/home/kathleen/.bash_history" => mark("ctf_history"),
    "/home/kathleen/.ssh/vault_key" => mark("vault_key"),
    _ => {}
  }
}

/// read a file the way cat does: Ok(lines) or Err(the error line)
fn read_file(cmd: &str, arg: &str) -> Result<Vec<String>, String> {
  let path = resolve(&cwd(), arg);
  match lookup(&path) {
    None => Err(err(format!("{}: {}: No such file or directory", cmd, arg))),
    Some(e) if e.kind == Kind::Dir => Err(err(format!("{}: {}: Is a directory", cmd, arg))),
    Some(e) if !can_read(e) => Err(err(format!("{}: {}: Permission denied", cmd, arg))),
    Some(e) => {
      on_read(&path);
      Ok((e.contents)())
    }
  }
}

/// the lines a filter command reads: its file args, or stdin
fn input_for(cmd: &str, files: &[&String], stdin: &Option<Vec<String>>) -> Vec<String> {
  if files.is_empty() {
    return stdin.clone().unwrap_or_default();
  }
  let mut out = vec![];
  for f in files {
    match read_file(cmd, f) {
      Ok(l) => out.extend(l),
      Err(e) => out.push(e),
    }
  }
  out
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

    if !can_read(entry) {
      out.push(err(format!("ls: cannot open directory '{}': Permission denied", arg)));
      continue;
    }

    if path == "/tmp" {
      mark("tmp");
    }

    if multiple {
      if i > 0 {
        out.push("".into());
      }
      out.push(format!("{}:", arg));
    }

    // sorted like `ls` with a normal locale: case-insensitive, ignoring the
    // leading dot. /tmp's .scratch always shows: it's the hunt's clue, and
    // the history hint is a plain `ls /tmp`
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

pub fn cat(args: &[String], stdin: &Option<Vec<String>>) -> Vec<String> {
  if args.is_empty() {
    return match stdin {
      Some(input) => input.clone(),
      None => vec!["usage: cat [file ...]".into()],
    };
  }

  let mut out = vec![];
  for arg in args {
    match read_file("cat", arg) {
      Ok(l) => out.extend(l),
      Err(e) => out.push(e),
    }
  }
  out
}

pub fn cd(args: &[String]) -> Vec<String> {
  let target = match args.first() {
    None => home().to_string(),
    Some(a) => resolve(&cwd(), a),
  };
  let shown = args.first().map(|s| s.as_str()).unwrap_or("~");

  match lookup(&target) {
    None => vec![err(format!("cd: no such file or directory: {}", shown))],
    Some(e) if e.kind == Kind::File => vec![err(format!("cd: not a directory: {}", shown))],
    Some(e) if !can_read(e) => vec![err(format!("cd: permission denied: {}", shown))],
    Some(_) => {
      set_cwd(target);
      vec![]
    }
  }
}

pub fn echo(args: &[String]) -> Vec<String> {
  vec![args.join(" ")]
}

pub fn env_vars() -> Vec<(&'static str, String)> {
  vec![
    ("EDITOR", "vim".into()),
    ("HOME", home().into()),
    ("LANG", "en_US.UTF-8".into()),
    ("LOGNAME", user().into()),
    ("PATH", "/usr/local/bin:/usr/bin:/bin".into()),
    ("PWD", cwd()),
    ("SHELL", "/bin/zsh".into()),
    ("TMPDIR", "/tmp".into()),
    ("USER", user().into()),
  ]
}

const BINARIES: &[&str] = &[
  "base64", "cat", "crontab", "curl", "date", "git", "grep", "hostname", "id", "less", "ls", "man", "mkdir", "nano",
  "bash", "gpg", "ps", "python3", "rev", "rm", "sh", "ssh", "su", "touch", "tr", "uname", "vim", "wc", "wget", "whoami", "xxd", "zsh",
];
const BUILTINS: &[&str] = &[".", "cd", "echo", "exec", "exit", "history", "pwd", "source", "which"];

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

pub fn crontab(args: &[String]) -> Vec<String> {
  match args.first().map(|s| s.as_str()) {
    Some("-l") if user() == "kathleen" => {
      mark("cron");
      vec!["0 3 * * * /opt/saudade/event_site/the_entertainment.sh  # do not watch".into()]
    }
    Some("-l") => vec![format!("crontab: no crontab for {}", user())],
    Some("-e") | Some("-r") => vec![err(format!("crontab: permission denied for {}", user()))],
    _ => vec!["usage: crontab [-e | -l | -r]".into()],
  }
}

/// `tr SET1 SET2` / `tr -d SET1`, with a-z style ranges
pub fn tr(args: &[String], stdin: &Option<Vec<String>>) -> Vec<String> {
  let expand_set = |s: &str| -> Vec<char> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = vec![];
    let mut i = 0;
    while i < cs.len() {
      if i + 2 < cs.len() && cs[i + 1] == '-' {
        let (a, b) = (cs[i] as u32, cs[i + 2] as u32);
        if a <= b {
          out.extend((a..=b).filter_map(char::from_u32));
        }
        i += 3;
      } else {
        out.push(cs[i]);
        i += 1;
      }
    }
    out
  };

  let input = stdin.clone().unwrap_or_default();
  match args {
    [flag, set] if flag == "-d" => {
      let set = expand_set(set);
      input.iter().map(|l| l.chars().filter(|c| !set.contains(c)).collect()).collect()
    }
    [from, to] => {
      let (from, to) = (expand_set(from), expand_set(to));
      if to.is_empty() {
        return vec![err("tr: when not truncating set1, string2 must be non-empty".into())];
      }
      input
        .iter()
        .map(|l| {
          l.chars()
            .map(|c| match from.iter().position(|f| *f == c) {
              Some(i) => *to.get(i).unwrap_or(to.last().unwrap()),
              None => c,
            })
            .collect()
        })
        .collect()
    }
    _ => vec!["usage: tr [-d] string1 [string2]".into()],
  }
}

const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn b64_encode(data: &[u8]) -> String {
  let mut out = String::new();
  for chunk in data.chunks(3) {
    let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
    let n = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
    for i in 0..4 {
      if i <= chunk.len() {
        out.push(B64[(n >> (18 - 6 * i) & 63) as usize] as char);
      } else {
        out.push('=');
      }
    }
  }
  out
}

fn b64_decode(s: &str) -> Option<Vec<u8>> {
  let mut bits = 0u32;
  let mut nbits = 0;
  let mut out = vec![];
  for c in s.bytes().filter(|c| !c.is_ascii_whitespace() && *c != b'=') {
    let v = B64.iter().position(|b| *b == c)? as u32;
    bits = bits << 6 | v;
    nbits += 6;
    if nbits >= 8 {
      nbits -= 8;
      out.push((bits >> nbits & 0xff) as u8);
    }
  }
  Some(out)
}

pub fn base64(args: &[String], stdin: &Option<Vec<String>>) -> Vec<String> {
  let decode = args.iter().any(|a| a == "-d" || a == "--decode" || a == "-D");
  let files: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();
  let input = input_for("base64", &files, stdin);
  if input.iter().any(|l| l.contains('\x1b')) {
    return input; // an error from reading the file
  }

  if decode {
    match b64_decode(&input.join("")) {
      Some(bytes) => {
        lines(&String::from_utf8_lossy(&bytes))
      }
      None => vec![err("base64: invalid input".into())],
    }
  } else {
    vec![b64_encode(input.join("\n").as_bytes())]
  }
}

/// `xxd` (hexdump), `xxd -p` (plain hex) and `xxd -r -p` (hex back to text)
pub fn xxd(args: &[String], stdin: &Option<Vec<String>>) -> Vec<String> {
  let reverse = args.iter().any(|a| a == "-r" || a.starts_with("-r"));
  let plain = args.iter().any(|a| a == "-p" || a == "-ps" || a == "-plain" || a == "-rp");
  let files: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();
  let input = input_for("xxd", &files, stdin);
  if input.iter().any(|l| l.contains('\x1b')) {
    return input;
  }

  if reverse {
    let digits: Vec<u8> = input.join("").bytes().filter(|c| c.is_ascii_hexdigit()).collect();
    let bytes: Vec<u8> = digits
      .chunks(2)
      .filter(|p| p.len() == 2)
      .filter_map(|p| u8::from_str_radix(std::str::from_utf8(p).ok()?, 16).ok())
      .collect();
    return lines(&String::from_utf8_lossy(&bytes));
  }

  let data = input.join("\n") + "\n";
  if plain {
    return hex(data.as_bytes()).as_bytes().chunks(60).map(|c| String::from_utf8_lossy(c).into()).collect();
  }
  data
    .as_bytes()
    .chunks(16)
    .enumerate()
    .map(|(i, chunk)| {
      let groups: Vec<String> = chunk.chunks(2).map(hex).collect();
      let text: String = chunk.iter().map(|b| if b.is_ascii_graphic() || *b == b' ' { *b as char } else { '.' }).collect();
      format!("{:08x}: {:<39}  {}", i * 16, groups.join(" "), text)
    })
    .collect()
}

pub fn grep(args: &[String], stdin: &Option<Vec<String>>) -> Vec<String> {
  let ignore_case = args.iter().any(|a| a == "-i");
  let rest: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();
  let (pattern, files) = match rest.split_first() {
    Some((p, f)) => (p.as_str(), f),
    None => return vec!["usage: grep [-i] pattern [file ...]".into()],
  };
  let input = input_for("grep", files, stdin);
  input
    .into_iter()
    .filter(|l| {
      if ignore_case { l.to_lowercase().contains(&pattern.to_lowercase()) } else { l.contains(pattern) }
    })
    .collect()
}

///// becoming kathleen

/// `su [-] [kathleen]`. anything aimed at root is handled before this (mr. robot)
pub fn su(args: &[String]) -> Vec<String> {
  let target = args.iter().find(|a| *a != "-" && *a != "-l").map(|s| s.as_str()).unwrap_or("root");
  match target {
    "kathleen" if user() == "kathleen" => vec![],
    "kathleen" => {
      await_input(Prompt::Su);
      vec![]
    }
    "stranger" if user() == "kathleen" => {
      logout();
      vec![]
    }
    other => vec![err(format!("su: user {} does not exist or the user entry does not contain all the required fields", other))],
  }
}

fn normalized(s: &str) -> String {
  s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
}

/// the line typed at a hidden prompt (su's password, gpg's passphrase).
/// returns the output to show
pub fn submit_password(attempt: &str) -> Vec<String> {
  let prompt = STATE.with(|s| s.borrow_mut().awaiting.take());
  if prompt == Some(Prompt::Gpg) {
    return open_vault(attempt);
  }

  if normalized(attempt) == PASSWORD {
    STATE.with(|s| s.borrow_mut().kathleen = true);
    set_cwd(KATHLEEN_HOME.into());
    mark("kathleen");
    vec![]
  } else {
    vec![err("su: Authentication failure".into())]
  }
}

fn logout() {
  STATE.with(|s| {
    let mut s = s.borrow_mut();
    s.kathleen = false;
    s.cwd = None;
  });
}

pub fn exit() -> Vec<String> {
  if user() == "kathleen" {
    logout();
    vec!["logout".into()]
  } else {
    vec!["logout".into(), dim("...nice try. there's no leaving :)")]
  }
}

/// remember a command line (for `history` and the up arrow)
pub fn record(line: &str) {
  let line = line.trim();
  if line.is_empty() {
    return;
  }
  STATE.with(|s| {
    let mut s = s.borrow_mut();
    // like zsh's HIST_IGNORE_DUPS
    if s.typed.last().map(|l| l.as_str()) != Some(line) {
      s.typed.push(line.to_string());
    }
  });
}

/// the commands typed this session, oldest first
pub fn typed() -> Vec<String> {
  STATE.with(|s| s.borrow().typed.clone())
}

/// `history`: whoever was here before you, then what you've typed
pub fn history() -> Vec<String> {
  let mut out: Vec<String> = if user() == "kathleen" {
    mark("ctf_history");
    kathleen_history()
  } else {
    mark("history");
    crate::content::HISTORY_HINT.iter().map(|s| s.to_string()).filter(|l| !l.trim().is_empty()).collect()
  };

  // keep numbering on from the last line shown
  let next = out
    .last()
    .and_then(|l| l.split_whitespace().next()?.parse::<usize>().ok())
    .map(|n| n + 1)
    .unwrap_or(1);
  let width = out.last().map(|l| l.len() - l.trim_start().len() + l.trim_start().find(' ').unwrap_or(0)).unwrap_or(5);
  for (i, cmd) in typed().iter().enumerate() {
    out.push(format!("{:>w$}  {}", next + i, cmd, w = width));
  }
  out
}

///// hints

// the first hint for each step is a line from one of kathleen's favorite books;
// the next two are plainer nudges
enum Hint {
  Quote(&'static str, &'static str),
  Text(&'static str),
}

// (milestone that completes the step, its hints from oblique to blunt)
type Step = (&'static str, [Hint; 3]);

const STEPS: &[Step] = &[
  ("read_me", [
    Hint::Quote(
      "Everything I can't see\nis at least as real as what I can.",
      "Stephen Dunn, \"Returning from an Artist's Studio\"",
    ),
    Hint::Text("`ls -a` shows the hidden files. one of them is meant to be read."),
    Hint::Text("try: cat .read_me"),
  ]),
  ("history", [
    // the .read_me itself is this step's quote, so don't repeat it
    Hint::Text("reread the .read_me. it's telling you which command to run."),
    Hint::Text("your shell remembers what was typed here before you arrived."),
    Hint::Text("try: history"),
  ]),
  ("tmp", [
    Hint::Quote(
      "Everything in this city looks like a stage set that could be taken down by tomorrow morning.",
      "Gary Indiana, Resentment",
    ),
    Hint::Text("temporary things have a folder of their own."),
    Hint::Text("try: ls /tmp"),
  ]),
  ("scratch", [
    Hint::Text("you found it. now read it."),
    Hint::Text("cat works on hidden files too."),
    Hint::Text("try: cat /tmp/.scratch"),
  ]),
  ("kathleen", [
    Hint::Quote(
      "None of us is anyone any more. We all lost who we were. But we can all be something else...",
      "Vic Serotonin, Nova Swing",
    ),
    Hint::Text("the fish is trout. the about page tells you how she came to stay."),
    Hint::Text("try: su kathleen (the password is fosterfail)"),
  ]),
  ("treasure", [
    Hint::Quote(
      "I look at the room with its familiar things, but today they seem strange to me, as if they belonged to someone else.",
      "Fernando Pessoa, The Book of Disquiet",
    ),
    Hint::Text("look around /home/kathleen."),
    Hint::Text("try: cat ~/treasure.txt"),
  ]),
];

// the ctf's milestones: tracked, but no hints
const CTF_STEPS: &[&str] = &["ctf_history", "cron", "vault_key", "vault"];

thread_local! {
  // the terminal's width, for wrapping blockquotes (set by `run`)
  static WIDTH: RefCell<usize> = RefCell::new(80);
}

/// a markdown-style blockquote: dim `>` on every line, then the attribution.
/// `\n` in the quote keeps a line break (for verse)
fn quote(text: &str, who: &str) -> Vec<String> {
  let width = WIDTH.with(|w| *w.borrow()).saturating_sub(2).max(20);
  let mut out: Vec<String> = format!("“{}”", text)
    .split('\n')
    .flat_map(|line| crate::md::wrap(line, width))
    .map(|l| format!("{} {}", dim(">"), l))
    .collect();
  out.push(dim(&format!("  — {}", who)));
  out
}

/// `hint --reset` (not mentioned anywhere): forget all hunt and ctf progress,
/// both solve-time clocks, and log back out to stranger. typed history stays
pub fn reset_hunt() -> Vec<String> {
  STATE.with(|s| {
    let mut s = s.borrow_mut();
    s.progress.clear();
    s.hints_shown.clear();
    s.kathleen = false;
    s.awaiting = None;
    s.cwd = None;
  });
  storage::reset();
  vec![dim("hunt progress cleared. you're a stranger again.")]
}

/// the step you're on: the one after the furthest you've gotten, so skipping
/// ahead (say, knowing the password) doesn't leave hints stuck on step one
fn current_step() -> Option<&'static Step> {
  let next = STEPS.iter().rposition(|(m, _)| reached(m)).map(|i| i + 1).unwrap_or(0);
  let step = STEPS.get(next)?;
  // the last step needs you to *be* kathleen; if you've logged back out, the
  // step is getting back in
  if step.0 == "treasure" && user() != "kathleen" {
    return STEPS.iter().find(|(m, _)| *m == "kathleen");
  }
  Some(step)
}

pub fn hint() -> Vec<String> {
  let step = match current_step() {
    Some(step) => step,
    None => {
      return vec![dim(if reached("vault") {
        "you've seen everything. there's nothing left out there but the tract."
      } else {
        "the hunt is done. but there's a vault on this machine, if you're the type. you're on your own for that one."
      })];
    }
  };

  let shown = STATE.with(|s| s.borrow().hints_shown.iter().find(|(m, _)| *m == step.0).map(|(_, n)| *n).unwrap_or(0));
  let i = shown.min(2);
  STATE.with(|s| {
    let mut s = s.borrow_mut();
    s.hints_shown.retain(|(m, _)| *m != step.0);
    s.hints_shown.push((step.0, shown + 1));
  });

  let label = format!("hint {}/3:", i + 1);
  match &step.1[i] {
    Hint::Text(text) => vec![dim(&format!("{} {}", label, text))],
    Hint::Quote(text, who) => {
      let mut out = vec![dim(&label)];
      out.extend(quote(text, who));
      out
    }
  }
}

///// running a line

/// `gpg -d .vault` (prompts for the passphrase) or
/// `gpg --batch --passphrase "..." -d .vault`
pub fn gpg(args: &[String]) -> Vec<String> {
  let decrypt = args.iter().any(|a| a == "-d" || a == "--decrypt");
  let mut passphrase: Option<String> = None;
  let mut file: Option<&String> = None;
  let mut i = 0;
  while i < args.len() {
    let a = &args[i];
    if let Some(p) = a.strip_prefix("--passphrase=") {
      passphrase = Some(p.to_string());
    } else if a == "--passphrase" {
      passphrase = args.get(i + 1).cloned();
      i += 1;
    } else if !a.starts_with('-') {
      file = Some(a);
    }
    i += 1;
  }

  let file = match (decrypt, file) {
    (true, Some(f)) => f,
    (true, None) => return vec!["usage: gpg --decrypt [file]".into()],
    (false, _) => return vec!["usage: gpg [options] [file]  (try: gpg --decrypt file)".into()],
  };

  let path = resolve(&cwd(), file);
  match lookup(&path) {
    None => return vec![err(format!("gpg: can't open '{}': No such file or directory", file))],
    Some(e) if e.kind == Kind::Dir => return vec![err(format!("gpg: read error: Is a directory"))],
    Some(e) if !can_read(e) => return vec![err(format!("gpg: can't open '{}': Permission denied", file))],
    Some(_) => {}
  }
  if path != "/opt/saudade/event_site/.vault" {
    return vec![err("gpg: no valid OpenPGP data found.".into()), err("gpg: decrypt_message failed: Unknown system error".into())];
  }

  match passphrase {
    Some(p) => {
      let mut out = vec![dim("gpg: AES256.CFB encrypted data")];
      out.extend(open_vault(&p));
      out
    }
    None => {
      await_input(Prompt::Gpg);
      vec![dim("gpg: AES256.CFB encrypted data")]
    }
  }
}

fn open_vault(attempt: &str) -> Vec<String> {
  if normalized(attempt) == normalized(PASSPHRASE) {
    mark("vault");
    let mut out = vec![dim("gpg: encrypted with 1 passphrase")];
    out.extend(vault_plaintext());
    out
  } else {
    vec![err("gpg: decryption failed: Bad session key".into())]
  }
}

/// running a file by its path (`./x`, `/opt/.../x`): nothing here is executable
fn run_path(path: &str) -> Vec<String> {
  match lookup(&resolve(&cwd(), path)) {
    None => vec![err(format!("zsh: no such file or directory: {}", path))],
    Some(e) if e.kind == Kind::Dir => vec![err(format!("zsh: permission denied: {}", path))],
    Some(_) => vec![err(format!("zsh: permission denied: {}", path))],
  }
}

/// `sh file`: "run" it a line at a time like a shell would, which mostly means
/// finding out that the lines aren't commands
fn run_script(cmd: &str, args: &[String]) -> Vec<String> {
  let file = match args.first() {
    Some(f) => f,
    // an interactive subshell; we're already in one
    None => return vec![],
  };
  let contents = match read_file(cmd, file) {
    Ok(lines) => lines,
    Err(e) => return vec![e],
  };

  let mut out = vec![];
  for (n, line) in contents.iter().enumerate() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
      continue;
    }
    let words = split_args(line);
    let target = words.get(1).map(|s| s.as_str()).unwrap_or("");
    out.push(err(match words[0].as_str() {
      "exec" if !target.is_empty() => format!("{}: line {}: {}: Permission denied", file, n + 1, target),
      first => format!("{}: line {}: {}: command not found", file, n + 1, first),
    }));
  }
  out
}

/// the rest of the commands, by name. `None` = not a command we know
pub fn simple(cmd: &str, args: &[String], stdin: &Option<Vec<String>>) -> Option<Vec<String>> {
  let first = args.first().map(|s| s.as_str()).unwrap_or("");
  let each = |msg: &dyn Fn(&str) -> String, missing: &str| -> Vec<String> {
    if args.is_empty() { vec![err(missing.to_string())] } else { args.iter().map(|a| err(msg(a))).collect() }
  };
  let uid = if user() == "kathleen" { 1000 } else { 1001 };

  Some(match cmd {
    "pwd" => vec![cwd()],
    "whoami" => vec![user().into()],
    "id" => vec![format!("uid={0}({1}) gid={0}({1}) groups={0}({1})", uid, user())],
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
    "exit" | "logout" => exit(),
    "su" => su(args),
    // hidden: start the hunt (and the ctf) over
    "hint" if args.iter().any(|a| a == "--reset") => reset_hunt(),
    "hint" => hint(),
    "exec" if args.is_empty() => vec![],
    "exec" => return simple(&args[0], &args[1..], stdin).or_else(|| Some(run_path(&args[0]))),
    "sh" | "bash" | "zsh" | "source" | "." => run_script(cmd, args),
    _ if cmd.contains('/') => run_path(cmd),
    "history" => history(),
    "crontab" => crontab(args),
    "which" => which(args),
    "echo" => echo(args),
    "cat" => cat(args, stdin),
    "cd" => cd(args),
    "ls" => ls(args),
    "tr" => tr(args, stdin),
    "base64" => base64(args, stdin),
    "gpg" | "gpg2" => gpg(args),
    "xxd" => xxd(args, stdin),
    "grep" => grep(args, stdin),
    "rev" => input_for("rev", &args.iter().collect::<Vec<_>>(), stdin).iter().map(|l| l.chars().rev().collect()).collect(),
    "wc" => vec![input_for("wc", &args.iter().filter(|a| !a.starts_with('-')).collect::<Vec<_>>(), stdin).len().to_string()],
    _ => return None,
  })
}

/// run a pipeline (`a | b | c`). Err(command) if a command isn't one we know
pub fn run_pipeline(line: &str, date: &dyn Fn() -> String) -> Result<Vec<String>, String> {
  let mut stdin: Option<Vec<String>> = None;
  for stage in split_pipeline(line) {
    let words = split_args(&stage);
    let (cmd, args) = match words.split_first() {
      Some((c, a)) => (c.as_str(), a),
      None => return Ok(vec![err("zsh: parse error near `|'".into())]),
    };
    let out = match cmd {
      "date" => vec![date()],
      _ => simple(cmd, args, &stdin).ok_or_else(|| cmd.to_string())?,
    };
    // what flows down the pipe is plain text
    stdin = Some(out.iter().map(|l| strip_ansi(l)).collect());
  }

  let out = stdin.unwrap_or_default();
  if out.iter().any(|l| l.contains("goto kefahuchi")) {
    mark("vault");
  }
  Ok(out)
}

/// run a command line. returns false if it isn't a command we know
pub fn run(term: &Terminal, line: &str) -> bool {
  WIDTH.with(|w| *w.borrow_mut() = term.get_cols() as usize);
  match run_pipeline(line, &now) {
    Ok(out) => {
      for l in out {
        crate::md::writeln(term, &l);
      }
      true
    }
    Err(_) => false,
  }
}

pub fn command_not_found(term: &Terminal, line: &str) {
  let cmd = split_args(line).into_iter().next().unwrap_or_default();
  crate::md::writeln(term, &err(format!("zsh: command not found: {}", cmd)));
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

/// progress + timing kept in the browser's localStorage, so the hint command
/// remembers where you are and the treasure pages can show your solve time
mod storage {
  #[cfg(target_arch = "wasm32")]
  fn store() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
  }

  #[cfg(target_arch = "wasm32")]
  pub fn load_progress() -> Vec<String> {
    store()
      .and_then(|s| s.get_item("kf_hunt_progress").ok()?)
      .map(|v| v.split(',').filter(|m| !m.is_empty()).map(String::from).collect())
      .unwrap_or_default()
  }

  #[cfg(target_arch = "wasm32")]
  pub fn milestone(m: &str) {
    let s = match store() {
      Some(s) => s,
      None => return,
    };
    let mut progress = load_progress();
    if !progress.iter().any(|p| p == m) {
      progress.push(m.to_string());
    }
    let _ = s.set_item("kf_hunt_progress", &progress.join(","));

    // the hunt's clock starts at the first clue; the ctf's when you become kathleen
    let now = js_sys::Date::now().to_string();
    let start_key = if m == "kathleen" { "kf_ctf_started" } else { "kf_hunt_started" };
    if s.get_item(start_key).ok().flatten().is_none() {
      let _ = s.set_item(start_key, &now);
    }
  }

  #[cfg(target_arch = "wasm32")]
  pub fn reset() {
    if let Some(s) = store() {
      for key in [
        "kf_hunt_progress", "kf_hunt_started", "kf_ctf_started",
        "kf_hunt_solved_ms", "kf_ctf_solved_ms", "kf_hunt_count", "kf_ctf_count",
      ] {
        let _ = s.remove_item(key);
      }
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn reset() {}

  #[cfg(not(target_arch = "wasm32"))]
  pub fn load_progress() -> Vec<String> {
    vec![]
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn milestone(_m: &str) {}
}

#[cfg(test)]
mod tests {
  use super::*;

  fn args(s: &str) -> Vec<String> {
    split_args(s)
  }

  fn plain(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|l| strip_ansi(l)).collect()
  }

  fn sh(line: &str) -> Vec<String> {
    plain(run_pipeline(line, &|| "DATE".into()).unwrap_or_else(|cmd| vec![format!("UNKNOWN {}", cmd)]))
  }

  fn reset() {
    STATE.with(|s| *s.borrow_mut() = State::default());
  }

  #[test]
  fn resolves_paths() {
    reset();
    assert_eq!(resolve(STRANGER_HOME, ".ssh"), "/home/stranger/.ssh");
    assert_eq!(resolve(STRANGER_HOME, "./.ssh/"), "/home/stranger/.ssh");
    assert_eq!(resolve(STRANGER_HOME, "~/.read_me"), "/home/stranger/.read_me");
    assert_eq!(resolve(STRANGER_HOME, "../.."), "/");
    assert_eq!(resolve("/tmp", "../home/stranger"), "/home/stranger");
    assert_eq!(resolve(STRANGER_HOME, "/tmp/.scratch"), "/tmp/.scratch");
  }

  #[test]
  fn ls_home() {
    reset();
    for home in ["", ".", "~", "/home/stranger", "/home/stranger/"] {
      assert_eq!(sh(&format!("ls {}", home)), vec!["Documents  Downloads  Pictures"], "ls {}", home);
    }
    assert_eq!(sh("ls /home"), vec!["kathleen  stranger"]);
    assert_eq!(sh("ls -A"), vec![".bashrc  Documents  Downloads  Pictures  .read_me  .ssh  .vimrc"]);
    let long = sh("ls -lah");
    assert!(long[0].starts_with("total "));
    assert!(long.iter().any(|l| l.starts_with("drwx------  2 kathleen kathleen  4.0K") && l.ends_with(" .ssh")));
  }

  #[test]
  fn ls_private_dirs_are_denied_with_or_without_flags() {
    reset();
    for cmd in [".ssh", "-la .ssh", "-lah ~/.ssh/", "Documents", "/root", "/home/kathleen", "/opt/saudade"] {
      let out = sh(&format!("ls {}", cmd));
      assert_eq!(out.len(), 1, "{}", cmd);
      assert!(out[0].ends_with("Permission denied"), "{}: {:?}", cmd, out);
    }
  }

  #[test]
  fn ls_errors() {
    reset();
    assert_eq!(sh("ls nope"), vec!["ls: cannot access 'nope': No such file or directory"]);
    assert_eq!(sh("ls -z")[0], "ls: invalid option -- 'z'");
    assert_eq!(sh("ls .bashrc"), vec![".bashrc"]);
  }

  #[test]
  fn cat_errors() {
    reset();
    assert_eq!(sh("cat .ssh"), vec!["cat: .ssh: Is a directory"]);
    assert_eq!(sh("cat .bashrc"), vec!["cat: .bashrc: Permission denied"]);
    assert_eq!(sh("cat nope"), vec!["cat: nope: No such file or directory"]);
  }

  #[test]
  fn cd_and_pwd() {
    reset();
    assert_eq!(sh("cd Documents"), vec!["cd: permission denied: Documents"]);
    assert_eq!(sh("cd nope"), vec!["cd: no such file or directory: nope"]);
    assert_eq!(sh("cd .bashrc"), vec!["cd: not a directory: .bashrc"]);
    assert!(sh("cd /tmp").is_empty());
    assert_eq!(sh("pwd"), vec!["/tmp"]);
    assert_eq!(sh("ls"), vec![".scratch"]);
    assert!(sh("cd").is_empty());
    assert_eq!(sh("pwd"), vec![STRANGER_HOME]);
  }

  #[test]
  fn echo_which_and_pipes() {
    reset();
    assert_eq!(sh("echo hi $USER, welcome to \"$HOME\""), vec!["hi stranger, welcome to /home/stranger"]);
    assert_eq!(sh("echo '$HOME' ${USER}x $NOPE."), vec!["$HOME strangerx ."], "single quotes stay literal");
    assert!(sh("cd /tmp").is_empty());
    assert!(sh("cd $HOME").is_empty(), "variables expand for every command");
    assert_eq!(sh("pwd"), vec![STRANGER_HOME]);
    assert_eq!(sh("which ls cd nope"), vec!["/usr/bin/ls", "cd: shell built-in command", "nope not found"]);
    assert_eq!(sh("echo hello | tr a-z A-Z"), vec!["HELLO"]);
    assert_eq!(sh("echo hello | base64 | base64 -d"), vec!["hello"]);
    assert_eq!(sh("echo stressed | rev"), vec!["desserts"]);
    assert_eq!(sh("cat .read_me | grep -i delillo"), vec!["― Don DeLillo, End Zone"]);
    assert_eq!(sh("frobnicate | cat"), vec!["UNKNOWN frobnicate"]);
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
  }

  #[test]
  fn the_hunt() {
    reset();
    let first = sh("hint");
    assert_eq!(first[1], "> “Everything I can't see");
    assert_eq!(first[2], "> is at least as real as what I can.”");
    assert_eq!(first[3], "  — Stephen Dunn, \"Returning from an Artist's Studio\"");
    assert!(sh("hint")[0].starts_with("hint 2/3"));
    assert!(sh("cat .read_me").iter().any(|l| l.contains("History is the angle")));
    assert_eq!(sh("hint"), vec!["hint 1/3: reread the .read_me. it's telling you which command to run."], "hints move on once a step is done");
    assert!(sh("history").iter().any(|l| l.contains("ls /tmp")));
    sh("ls /tmp");
    let scratch = sh("cat /tmp/.scratch");
    assert!(scratch.iter().any(|l| l == "pw = 🐟 origin story (one word, no spaces)"));

    assert!(sh("su kathleen").is_empty());
    assert!(is_awaiting_password());
    assert_eq!(plain(submit_password("trout")), vec!["su: Authentication failure"]);
    assert_eq!(user(), "stranger");

    sh("su kathleen");
    assert!(plain(submit_password("Foster Fail")).is_empty());
    assert_eq!(user(), "kathleen");
    assert_eq!(sh("pwd"), vec![KATHLEEN_HOME]);
    assert_eq!(sh("whoami"), vec!["kathleen"]);
    assert!(sh("cat treasure.txt").iter().any(|l| l.contains("goto l00t")));
    assert!(sh("hint")[0].contains("vault"));

    assert_eq!(sh("exit"), vec!["logout"]);
    assert_eq!(user(), "stranger");
  }

  #[test]
  fn the_ctf() {
    reset();
    assert!(!vault_key()[0].to_lowercase().contains("motel"), "the key doesn't give the answer away");
    assert_eq!(sh("gpg -d nope"), vec!["gpg: can't open 'nope': No such file or directory"]);
    sh("su kathleen");
    submit_password("fosterfail");
    assert!(sh("cat ~/.bash_history").iter().any(|l| l.contains("watchman")));
    assert!(sh("crontab -l")[0].contains("the_entertainment.sh"));
    let script = sh("cat /opt/saudade/event_site/the_entertainment.sh").join("\n");
    assert!(script.contains("gpg --batch") && !script.contains("vault_key"), "the script shows gpg, not the key");
    let layer = sh("base64 -d ~/.ssh/vault_key");
    assert!(layer[0].starts_with("70773a"), "base64 hides hex: {:?}", layer);
    assert_eq!(sh("base64 -d ~/.ssh/vault_key | xxd -r -p").join("\n"), VAULT_RIDDLE);
    assert_eq!(sh("ls -a /opt/saudade/event_site"), vec![".  ..  README  the_entertainment.sh  .vault"]);
    assert_eq!(sh("cat /opt/saudade/event_site/.vault")[0], "-----BEGIN PGP MESSAGE-----");

    // interactive: prompts for the passphrase
    sh("cd /opt/saudade/event_site");
    assert_eq!(sh("gpg -d .vault"), vec!["gpg: AES256.CFB encrypted data"]);
    assert_eq!(password_prompt(), "Enter passphrase: ");
    assert_eq!(plain(submit_password("nope")), vec!["gpg: decryption failed: Bad session key"]);
    assert!(!reached("vault"));
    sh("gpg --decrypt .vault");
    let opened = plain(submit_password("Motel Splendido"));
    assert_eq!(opened, vec!["gpg: encrypted with 1 passphrase", "you crossed it.", "goto kefahuchi"]);
    assert!(reached("vault"));

    // non-interactive, like the cron job
    assert_eq!(
      sh(&format!("gpg --batch --passphrase \"{}\" -d .vault", PASSPHRASE))[2..],
      ["you crossed it.", "goto kefahuchi"],
    );
    assert_eq!(sh("gpg -d README"), vec!["gpg: no valid OpenPGP data found.", "gpg: decrypt_message failed: Unknown system error"]);
  }

  #[test]
  fn stranger_cant_reach_the_ctf() {
    reset();
    assert_eq!(sh("crontab -l"), vec!["crontab: no crontab for stranger"]);
    assert_eq!(sh("cat /opt/saudade/event_site/.vault"), vec!["cat: /opt/saudade/event_site/.vault: Permission denied"]);
    assert_eq!(sh("su root"), vec!["su: user root does not exist or the user entry does not contain all the required fields"]);
  }

  #[test]
  fn quote_hints_wrap_as_blockquotes() {
    reset();
    WIDTH.with(|w| *w.borrow_mut() = 40);
    let q = plain(quote("None of us is anyone any more. We all lost who we were. But we can all be something else...", "Vic Serotonin, Nova Swing"));
    assert!(q.len() > 2);
    assert!(q[..q.len() - 1].iter().all(|l| l.starts_with("> ") && l.chars().count() <= 40), "{:?}", q);
    assert_eq!(q.last().unwrap(), "  — Vic Serotonin, Nova Swing");
    WIDTH.with(|w| *w.borrow_mut() = 80);
  }

  #[test]
  fn history_includes_what_you_typed() {
    reset();
    record("ls -a");
    record("ls -a");
    record("cat .read_me");
    let h = sh("history");
    assert_eq!(h[h.len() - 2].trim(), "9935  ls -a", "{:?}", h);
    assert_eq!(h[h.len() - 1].trim(), "9936  cat .read_me");
    assert_eq!(h[h.len() - 1].find("cat"), h[0].find("ls"), "commands line up");
    assert_eq!(typed(), vec!["ls -a", "cat .read_me"]);
  }

  #[test]
  fn hint_reset_starts_over() {
    reset();
    sh("cat .read_me");
    sh("su kathleen");
    submit_password("fosterfail");
    assert_eq!(user(), "kathleen");
    assert_eq!(sh("hint --reset"), vec!["hunt progress cleared. you're a stranger again."]);
    assert_eq!(user(), "stranger");
    assert_eq!(sh("pwd"), vec![STRANGER_HOME]);
    assert!(sh("hint")[1].contains("Everything I can't see"), "back to step one");
  }

  #[test]
  fn hints_follow_your_furthest_progress() {
    reset();
    sh("su kathleen");
    submit_password("fosterfail");
    assert!(sh("hint").join(" ").contains("Fernando Pessoa"), "skipped straight to the last step");
    sh("exit");
    assert!(sh("hint").join(" ").contains("Vic Serotonin"), "logged out: get back in first");
  }

  #[test]
  fn running_files() {
    reset();
    sh("su kathleen");
    submit_password("fosterfail");
    sh("cd /opt/saudade/event_site");
    assert_eq!(sh("exec ./.vault"), vec!["zsh: permission denied: ./.vault"]);
    assert_eq!(sh("./the_entertainment.sh"), vec!["zsh: permission denied: ./the_entertainment.sh"]);
    assert_eq!(sh("./nope"), vec!["zsh: no such file or directory: ./nope"]);
    assert_eq!(sh("sh .vault")[0], ".vault: line 1: -----BEGIN: command not found");
    assert_eq!(sh("exec pwd"), vec!["/opt/saudade/event_site"]);
    assert!(sh("sh").is_empty());
  }

  #[test]
  fn prompt_shows_the_directory() {
    reset();
    assert_eq!(prompt_dir(), "~");
    sh("cd /tmp");
    assert_eq!(prompt_dir(), "/tmp");
    sh("su kathleen");
    submit_password("fosterfail");
    assert_eq!(prompt_dir(), "~");
    sh("cd .ssh");
    assert_eq!(prompt_dir(), "~/.ssh");
    sh("cd /opt/saudade/event_site");
    assert_eq!(prompt_dir(), "/opt/saudade/event_site");
  }

  #[test]
  fn xxd_round_trips() {
    reset();
    assert_eq!(sh("echo hi there | xxd -p"), vec!["68692074686572650a"], "echo's newline included, like real xxd");
    assert_eq!(sh("echo 6869207468657265 | xxd -r -p"), vec!["hi there"]);
    assert_eq!(sh("echo hi | xxd"), vec!["00000000: 6869 0a                                  hi."]);
  }

  #[test]
  fn command_substitution() {
    reset();
    assert_eq!(sh("echo $(whoami) is in $(pwd)"), vec!["stranger is in /home/stranger"]);
    assert_eq!(sh("echo $(echo aGk= | base64 -d)"), vec!["hi"], "pipes inside $()");
    assert_eq!(sh("echo $(echo aGk=) | base64 -d"), vec!["hi"]);
    assert_eq!(sh("cat $(echo nope)"), vec!["cat: nope: No such file or directory"], "output becomes the argument, like a real shell");
    assert_eq!(sh("echo '$(whoami)'"), vec!["$(whoami)"], "not inside single quotes");
  }

  #[test]
  fn unknown_commands_fall_through() {
    reset();
    assert!(simple("frobnicate", &[], &None).is_none());
  }
}
