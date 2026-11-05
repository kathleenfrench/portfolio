// markdown-flavored terminal output: dim markup characters (#, *, table
// borders) around bold uppercase headings and plain text
use ansi_term::{Colour, Style};
use xterm_js_rs::Terminal;

pub const DIM: Colour = Colour::Fixed(244);

/// `## TITLE` followed by a blank line
pub fn heading(term: &Terminal, title: &str) {
  writeln(term, &format!("{} {}", DIM.paint("##"), Style::new().bold().paint(title.to_uppercase())));
  writeln(term, "");
}

/// `  * text`
pub fn bullet(term: &Terminal, text: &str) {
  writeln(term, &format!("  {} {}", DIM.paint("*"), text));
}

/// `  * name      description`, with names padded to `width`
pub fn item(term: &Terminal, name: &str, width: usize, desc: &str) {
  bullet(term, &format!("{:<width$}{}", name, desc, width = width));
}

/// dim text for hints and footnotes
pub fn note(term: &Terminal, text: &str) {
  writeln(term, &DIM.paint(text).to_string());
}

/// write text to the terminal, word-wrapped to its width. everything the
/// terminal prints should go through here (except the intro, whose log lines
/// are meant to run off the edge)
pub fn writeln(term: &Terminal, text: &str) {
  let width = term.get_cols() as usize;
  for line in text.split('\n') {
    for wrapped in wrap_ansi(line, width) {
      term.writeln(&wrapped);
    }
  }
}

/// word-wrap one line that may contain color codes. the codes pass through
/// untouched (the terminal keeps the current color across a line break), and
/// continuation lines get a hanging indent so lists and labels stay aligned
pub fn wrap_ansi(line: &str, width: usize) -> Vec<String> {
  if width < 20 || strip_len(line) <= width {
    return vec![line.to_string()];
  }

  let indent = hanging_indent(&strip(line), width);
  let mut lines = vec![];
  let mut cur = String::new();
  let mut col = 0;
  // byte index in `cur` of the last breakable space, and its column
  let mut last_space: Option<(usize, usize)> = None;
  let mut in_escape = false;

  for c in line.chars() {
    if in_escape || c == '\x1b' {
      in_escape = c != 'm';
      cur.push(c);
      continue;
    }
    // a continuation line starts at its indent, not with leftover spaces
    if c == ' ' && !lines.is_empty() && col == indent {
      continue;
    }
    if c == ' ' {
      last_space = Some((cur.len(), col));
    }
    cur.push(c);
    col += 1;

    if col > width {
      match last_space {
        Some((at, space_col)) if space_col > indent => {
          let rest = cur[at + 1..].trim_start_matches(' ').to_string();
          cur.truncate(at);
          let kept = cur.trim_end_matches(' ').len();
          cur.truncate(kept);
          lines.push(std::mem::take(&mut cur));
          col = indent + strip_len(&rest);
          cur = " ".repeat(indent) + &rest;
        }
        // one long word: break it
        _ => {
          let last = cur.pop().unwrap_or(' ');
          lines.push(std::mem::take(&mut cur));
          cur = format!("{}{}", " ".repeat(indent), last);
          col = indent + 1;
        }
      }
      last_space = None;
    }
  }
  lines.push(cur);
  lines
}

// how far to indent continuation lines: under the text of a `  * bullet` or a
// `> quote`, under
// the details of a `label    - details` line, or the line's own indentation
fn hanging_indent(plain: &str, width: usize) -> usize {
  let lead = plain.len() - plain.trim_start().len();
  let rest = plain.trim_start();
  let indent = if rest.starts_with("* ") || rest.starts_with("- ") || rest.starts_with("> ") {
    lead + 2
  } else {
    match plain.find(" - ") {
      Some(at) if at < 40 => at + 3,
      _ => lead,
    }
  };
  if indent > width / 2 { 0 } else { indent }
}

fn strip(s: &str) -> String {
  let mut out = String::new();
  let mut in_escape = false;
  for c in s.chars() {
    match (in_escape, c) {
      (false, '\x1b') => in_escape = true,
      (true, 'm') => in_escape = false,
      (true, _) => {}
      (false, c) => out.push(c),
    }
  }
  out
}

/// split text into lines of at most `width` chars, breaking between words
/// (a single word longer than `width` gets a line to itself)
pub fn wrap(text: &str, width: usize) -> Vec<String> {
  let mut lines = vec![];
  let mut line = String::new();

  for word in text.split_whitespace() {
    if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
      lines.push(std::mem::take(&mut line));
    }
    if !line.is_empty() {
      line.push(' ');
    }
    line.push_str(word);
  }

  if !line.is_empty() || lines.is_empty() {
    lines.push(line);
  }
  lines
}

/// an ascii table. column widths are measured on the plain text, then
/// `style` can color a cell (it gets the column index) without breaking them
pub fn table(term: &Terminal, header: &[&str], rows: &[Vec<String>], style: impl Fn(usize, &str) -> String) {
  let mut widths: Vec<usize> = header.iter().map(|h| h.len()).collect();
  for row in rows {
    for (i, cell) in row.iter().enumerate() {
      widths[i] = widths[i].max(cell.chars().count());
    }
  }

  let border = DIM
    .paint(format!("+{}+", widths.iter().map(|w| "-".repeat(w + 2)).collect::<Vec<_>>().join("+")))
    .to_string();
  let pipe = DIM.paint("|").to_string();

  let line = |cells: Vec<String>| {
    let padded: Vec<String> = cells.iter().enumerate()
      .map(|(i, c)| format!(" {}{} ", c, " ".repeat(widths[i] - strip_len(c))))
      .collect();
    format!("{}{}{}", pipe, padded.join(&pipe), pipe)
  };

  writeln(term, &border);
  writeln(term, &line(header.iter().map(|h| Style::new().bold().paint(*h).to_string()).collect()));
  writeln(term, &border);
  for row in rows {
    writeln(term, &line(row.iter().enumerate().map(|(i, c)| style(i, c)).collect()));
  }
  writeln(term, &border);
}

// visible length of a string that may contain ansi color codes
fn strip_len(s: &str) -> usize {
  let mut len = 0;
  let mut in_escape = false;
  for c in s.chars() {
    match (in_escape, c) {
      (false, '\x1b') => in_escape = true,
      (true, 'm') => in_escape = false,
      (true, _) => {}
      (false, _) => len += 1,
    }
  }
  len
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wrap_breaks_between_words() {
    assert_eq!(
      wrap("last login: wed nov 4 21:34:32 from somewhere on the internet", 50),
      vec!["last login: wed nov 4 21:34:32 from somewhere on", "the internet"],
    );
    assert_eq!(wrap("short", 50), vec!["short"]);
    assert_eq!(wrap("", 50), vec![""]);
  }

  #[test]
  fn wrap_ansi_keeps_colors_and_hangs_indents() {
    let lede = format!("{} {}", Colour::White.bold().paint("CI/CD|"), "            - buildkite, jenkins, circleci, argo, flux, flagger, heroku, github actions");
    let wrapped = wrap_ansi(&lede, 60);
    assert!(wrapped.len() > 1);
    assert!(wrapped.iter().all(|l| strip_len(l) <= 60), "{:?}", wrapped);
    let col = strip(&wrapped[0]).find("buildkite").unwrap();
    assert!(strip(&wrapped[1]).starts_with(&" ".repeat(col)), "continues under the details: {:?}", wrapped);
    assert!(wrapped[0].contains('\x1b'), "color codes are kept");

    let bullet = wrap_ansi("  * a fairly long bullet point that will definitely need to wrap somewhere", 30);
    assert!(bullet[1].starts_with("    "), "continues under the bullet text: {:?}", bullet);
    assert_eq!(wrap_ansi("short", 30), vec!["short"]);

    // column padding doesn't carry over to the next line
    let edu = wrap_ansi("NEW YORK UNIVERSITY    - m.f.a in fiction                    [2015]", 50);
    assert_eq!(strip(&edu[1]).trim_start(), "[2015]");
    assert_eq!(strip(&edu[1]).len() - "[2015]".len(), strip(&edu[0]).find("m.f.a").unwrap());
  }

  #[test]
  fn strip_len_ignores_color_codes() {
    assert_eq!(strip_len("nvidia"), 6);
    assert_eq!(strip_len(&Colour::Cyan.bold().paint("nvidia").to_string()), 6);
  }
}
