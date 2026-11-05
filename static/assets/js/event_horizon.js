// an ascii black hole: a dark disc, a thin bright photon ring around it, and a
// tilted accretion disk swirling faster the closer it gets. redrawn into a
// <pre> about 20 times a second.
(function () {
  var pre = document.getElementById('horizon');
  var RAMP = ' .,:;-=+*%#@';
  var cols = 0, rows = 0, charW = 8, charH = 16;

  // measure one character so the grid fills the window exactly
  function measure() {
    var probe = document.createElement('span');
    probe.textContent = 'MMMMMMMMMM';
    pre.appendChild(probe);
    var box = probe.getBoundingClientRect();
    pre.removeChild(probe);
    charW = box.width / 10 || 8;
    charH = box.height || 16;
    cols = Math.ceil(window.innerWidth / charW);
    rows = Math.ceil(window.innerHeight / charH);
  }

  // cheap stable per-cell noise for the starfield
  function hash(x, y) {
    var h = Math.sin(x * 127.1 + y * 311.7) * 43758.5453;
    return h - Math.floor(h);
  }

  function frame(t) {
    var cx = cols / 2, cy = rows * 0.42;
    // radius in "x units"; y is scaled by the char aspect so the hole is round
    var R = Math.min(cols, rows * charH / charW) * 0.13;
    var aspect = charH / charW;
    var out = '';

    for (var y = 0; y < rows; y++) {
      for (var x = 0; x < cols; x++) {
        var px = x - cx, py = (y - cy) * aspect;
        var r = Math.sqrt(px * px + py * py);
        var b = 0;

        if (r < R) {
          out += ' ';
          continue;
        }

        // photon ring: thin and bright, flickering a little
        var ring = 1 - Math.abs(r - R * 1.06) / (R * 0.07);
        if (ring > 0) b = Math.max(b, ring * (0.85 + 0.15 * Math.sin(t * 3 + Math.atan2(py, px) * 6)));

        // accretion disk: a ring seen almost edge-on (squashed vertically).
        // the half behind the hole is hidden by it, the front half crosses over
        var dy = py * 4.2;
        var d = Math.sqrt(px * px + dy * dy);
        if (d > R * 1.25 && d < R * 3.6) {
          var angle = Math.atan2(dy, px);
          var speed = 2.2 * (R * 1.5) / d;            // inner orbits are faster
          var swirl = 0.55 + 0.45 * Math.sin(angle * 5 - t * speed + d * 0.35);
          var falloff = 1 - (d - R * 1.25) / (R * 2.35);
          b = Math.max(b, falloff * swirl * 0.95);
        }

        // a few stars, twinkling
        if (b === 0) {
          var s = hash(x, y);
          if (s > 0.993) b = 0.18 + 0.12 * Math.sin(t * 2 + s * 50);
        }

        out += RAMP.charAt(Math.max(0, Math.min(RAMP.length - 1, Math.floor(b * RAMP.length))));
      }
      out += '\n';
    }
    pre.textContent = out;
  }

  var still = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  var last = 0;

  function loop(now) {
    if (now - last > 50) {
      frame(now / 1000);
      last = now;
    }
    requestAnimationFrame(loop);
  }

  measure();
  window.addEventListener('resize', function () { measure(); frame(last / 1000); });
  if (still) frame(0);
  else requestAnimationFrame(loop);
})();
