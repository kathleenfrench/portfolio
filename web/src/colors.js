const colors = {
  blue: '\u001b[34m',
  red: '\x1b[1;31m',
  green: '\u001b[32m',
  white: '\u001b[37m',
  yellow: '\u001b[33m',
  cyan: '\u001b[36m',
  bold_yellow: '\u001b[33;1m',
  bold_blue: '\u001b[34;1m',
  bold_red: '\x1b[1;31;1m',
  bold_green: '\u001b[32;1m',
  bold_white: '\u001b[37;1m',
  bold_cyan: '\u001b[36;1m'
}

export function colorize(colorCode, text) {
  return `${colors[colorCode]}` + text + '\x1b[37m';
}