import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';

// the intro animation terminal. the wasm app (web/pterm) writes to it via
// `window.pterm` (see pterm/src/io.rs), so it has to exist before the wasm loads.
// one font for the whole site, terminals included. xterm measures character
// widths when a terminal opens, so wait for the web font or columns misalign
const FONT = "'IBM Plex Mono', monospace";
await Promise.all([
  document.fonts.load(`12px ${FONT}`),
  document.fonts.load(`bold 12px ${FONT}`),
]).catch(() => {});

const pterm = new Terminal({
  fontFamily: FONT,
  cursorBlink: true,
  fontSize: 12,
  cursorWidth: 8,
  rightClickSelectsWord: true,
  scrollback: 0,
});

const fitAddon = new FitAddon();
pterm.loadAddon(fitAddon);
pterm.open(document.getElementById('intro-terminal'));
fitAddon.fit();

window.pterm = pterm;

// built by `make wasm` (wasm-bindgen output); its start function boots the main terminal
import('../pkg');
