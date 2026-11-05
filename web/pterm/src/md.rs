// markdown-flavored terminal output: dim markup characters (#, *, table
// borders) around bold uppercase headings and plain text
use ansi_term::{Colour, Style};
use xterm_js_rs::Terminal;

pub const DIM: Colour = Colour::Fixed(244);

/// `## TITLE` followed by a blank line
pub fn heading(term: &Terminal, title: &str) {
  term.writeln(&format!("{} {}", DIM.paint("##"), Style::new().bold().paint(title.to_uppercase())));
  term.writeln("");
}

/// `  * text`
pub fn bullet(term: &Terminal, text: &str) {
  term.writeln(&format!("  {} {}", DIM.paint("*"), text));
}

/// `  * name      description`, with names padded to `width`
pub fn item(term: &Terminal, name: &str, width: usize, desc: &str) {
  bullet(term, &format!("{:<width$}{}", name, desc, width = width));
}

/// dim text for hints and footnotes, word-wrapped to the terminal width
pub fn note(term: &Terminal, text: &str) {
  for line in wrap(text, term.get_cols() as usize) {
    term.writeln(&DIM.paint(line).to_string());
  }
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

  term.writeln(&border);
  term.writeln(&line(header.iter().map(|h| Style::new().bold().paint(*h).to_string()).collect()));
  term.writeln(&border);
  for row in rows {
    term.writeln(&line(row.iter().enumerate().map(|(i, c)| style(i, c)).collect()));
  }
  term.writeln(&border);
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
  fn strip_len_ignores_color_codes() {
    assert_eq!(strip_len("nvidia"), 6);
    assert_eq!(strip_len(&Colour::Cyan.bold().paint("nvidia").to_string()), 6);
  }
}
