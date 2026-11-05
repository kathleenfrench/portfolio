// solve time + finder count for the treasure pages.
//
// the terminal (web/pterm/src/shell.rs) records when you started each puzzle in
// localStorage. here we freeze the solve time on your first arrival, and ask the
// server to count you once (it keeps the totals in upstash).
//
//   showFound('hunt', document.getElementById('found-stats'))
function showFound(track, el) {
  var store = {
    get: function (k) { try { return localStorage.getItem(k); } catch (e) { return null; } },
    set: function (k, v) { try { localStorage.setItem(k, v); } catch (e) {} },
  };

  var started = Number(store.get('kf_' + track + '_started'));
  var solved = Number(store.get('kf_' + track + '_solved_ms'));
  if (!solved && started) {
    solved = Date.now() - started;
    store.set('kf_' + track + '_solved_ms', String(solved));
  }

  function duration(ms) {
    var s = Math.round(ms / 1000);
    var h = Math.floor(s / 3600), m = Math.floor((s % 3600) / 60);
    if (h) return h + 'h ' + m + 'm';
    if (m) return m + 'm ' + (s % 60) + 's';
    return s + 's';
  }

  function ordinal(n) {
    var tens = n % 100, ones = n % 10;
    if (tens >= 11 && tens <= 13) return n + 'th';
    return n + (ones === 1 ? 'st' : ones === 2 ? 'nd' : ones === 3 ? 'rd' : 'th');
  }

  function render(count) {
    var parts = [];
    if (count) parts.push("you're the " + ordinal(count) + ' person to find this');
    if (solved) parts.push('solved in ' + duration(solved));
    el.textContent = parts.join(' · ');
    el.hidden = parts.length === 0;
  }

  // only ask the server to count you once; after that, show your number
  var known = Number(store.get('kf_' + track + '_count'));
  if (known) return render(known);

  render(null);
  fetch('/api/found/' + track, { method: 'POST' })
    .then(function (r) { return r.json(); })
    .then(function (body) {
      if (body.count) store.set('kf_' + track + '_count', String(body.count));
      render(body.count);
    })
    .catch(function () {});
}
