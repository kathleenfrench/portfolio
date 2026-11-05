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
  // keep in sync with pterm/src/lib.rs + main.css
  theme: { background: '#111111', foreground: '#e6e6e6', cursor: '#e6e6e6' },
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
// the intro is a fake log stream: never wrap, let long lines run off the right
// edge of the window like a real terminal (main.css clips it). wrapping also
// breaks the animation, which rewrites lines in place with \r
pterm.resize(Math.max(pterm.cols, 300), pterm.rows);
window.addEventListener('resize', () => {
  fitAddon.fit();
  pterm.resize(Math.max(pterm.cols, 300), pterm.rows);
});

window.pterm = pterm;

// built by `make wasm` (wasm-bindgen output); its start function boots the main terminal
import('../pkg');

// real server info for the terminal's `neofetch` and `who` (pterm/src/shell.rs
// reads window.kfSysinfo / window.kfWho). each tab checks in once a minute while
// it's visible, with a random id; the server keeps nothing else
window.kfSessionId = Math.random().toString(36).slice(2, 12);
fetch('/api/sysinfo').then((r) => r.json()).then((info) => { window.kfSysinfo = info; }).catch(() => {});

function checkIn() {
  if (document.hidden) return;
  fetch('/api/who', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ id: window.kfSessionId }),
  })
    .then((r) => r.json())
    .then((body) => { window.kfWho = body.sessions; })
    .catch(() => {});
}
checkIn();
setInterval(checkIn, 60 * 1000);
document.addEventListener('visibilitychange', checkIn);
