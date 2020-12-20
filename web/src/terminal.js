import { colorize } from './colors';

export const xtermOptions = {
  cursorBlink: true,
  fontSize: 12,
  cursorWidth: 8,
  rightClickSelectsWord: true,
  scrollback: 0,
}

export function generatePrompt() {
  let prompt = colorize("bold_green", `kathleenfrench@portfolio $ `);
  return prompt
}