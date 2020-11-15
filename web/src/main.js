import { A } from './common';

console.log("a: ", A);

const opener = [
  "hey, it looks like you found my website!",
  "one sec, let me prepare a few things...",
  "won't be much longer",
  "...didn't know you were coming haha",
]

const closer = [
  "ehh..this is taking longer than i thought -",
  "you know what, why don't you just take a look around?",
]

Terminal.applyAddon(terminado);
Terminal.applyAddon(fit);
Terminal.applyAddon(search);

var term = new Terminal({
  fontSize: 10,
  cursorBlink: true,
});
var protocol = (location.protocol === 'https:') ? 'wss://' : 'ws://';
var socketURL = protocol + location.hostname + ((location.port) ? (':' + location.port) : '') + "/ws/";
var sock = new WebSocket(socketURL);

sock.addEventListener('open', function(){
  console.log("[ws]: connection opened")
  term.fit();
  term.terminadoAttach(sock);
  term.writeln("");

  var i = 0, loopTimeout;
  function loopIntro() {
    loopTimeout = setTimeout(function(){
      if (i < opener.length) {
        term.writeln(opener[i]);
        i++;
        loopIntro();
      } else {
        killIntroLoop();
        term.clear();
        sock.send('print_faux_logs');
      }
    }, 1000);
  }

  function killIntroLoop(){
    clearTimeout(loopTimeout);
  }

  loopIntro();
})

sock.addEventListener('close', function(){
  console.log('[ws]: connection closed');
  term.writeln("");
  term.writeln("connection closed");
  term.terminadoDetach(sock);
})

sock.addEventListener('message', function(event) {
  console.log("[ws]: message from server: ", event);
  var parsed = JSON.parse(event.data);
  var responseKey = parsed.key;
  var responseMessage = parsed.message;

  switch(responseKey) {
    case "print_faux_logs":
      var i = 0;
      var loopLogs;
      var fakeLogs = responseMessage.split(`\n`);

      function loopFauxLogs() {
        loopLogs = setTimeout(() => {
          if (i < fakeLogs.length) {
            term.writeln(fakeLogs[i]);
            i++;
        
            if (i == 70) {
              term.clear();
              term.writeln("rebooting....")
            }

            if (i == 150) {
              term.clear();
              term.writeln("makin' it pretty");
              term.writeln("promise we're almost there...");
            }

            if (i == 200) {
              term.clear();
              term.writeln("powering up the server farm (jk)")
            }

            if (i == 250) {
              term.clear();
              term.writeln("juuuuuust one more second....");
            }

            loopFauxLogs()
          } else {
            clearTimeout(loopLogs);
            term.clear();
            term.write("kathleenfrench@portfolio $ ");
          }
        }, 20);
      }

      loopFauxLogs();
      break;
    default:
      term.write(event.data);
      break;
  }
})

sock.addEventListener('error', function(event) {
  console.error(`ws[] error ocurred: ${event}`)
})

term.open(document.getElementById('terminal'));
window.onresize = function() {term.fit();};