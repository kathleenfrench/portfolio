import { colorize } from './colors';

export const xtermOptions = {
  cursorBlink: true,
  fontSize: 12,
  cursorWidth: 8,
  rightClickSelectsWord: true,
}

export function generatePrompt() {
  let prompt = colorize("bold_green", `kathleenfrench@portfolio $ `);
  return prompt
}

export const opener = [
  "hey, i'm kathleen - it looks like you found my website!",
  "give me a sec to get organized...",
  "i won't take too long, just a few loose ends to tie up...",
]