import { A } from './common';
import { xtermOptions, generatePrompt } from './terminal';
import { colorize } from './colors';

console.log("a: ", A);

const opener = [
  "hey, it looks like you found my website!",
  "one sec, let me prepare a few things...",
  "won't take too long",
  "just have a couple loose ends to tie up first...",
]

Terminal.applyAddon(terminado);
Terminal.applyAddon(fit);
Terminal.applyAddon(search);

var term = new Terminal(xtermOptions);
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
    // TODO: add a 'you've been here before' based on the cookie, so you know the drill etc
    // but if you wanna see me start things up again, do 'replay'
    case "print_faux_logs":
      var i = 0;
      var loopLogs;
      var fakeLogs = responseMessage.split(`\n`);

      function loopFauxLogs() {
        loopLogs = setTimeout(() => {
          if (i < fakeLogs.length) {
            term.writeln(fakeLogs[i]);
            i++;

            if (i == 1) {
              term.writeln(colorize("bold_yellow", "initializing...."))
            }
        
            if (i == 20) {
              term.clear();
              term.writeln(colorize("bold_yellow", "powering up the server farm...."))
            }

            if (i == 50) {
              term.clear();
              term.writeln(colorize("bold_yellow", "provisioning v necessary 32 core instance..."));
            }

            if (i == 70) {
              term.clear();
              term.writeln(colorize("bold_yellow", "...btw pls sponsor my 32 core instance"));
            }

            if (i == 95) {
              term.clear();
              term.writeln(colorize("bold_yellow", "making things look pretty for the aesthetically inclined..."));
            }

            if (i == 120) {
              term.clear();
              term.writeln(colorize("bold_yellow", "juuuuuust one more second...."));
            }

            loopFauxLogs()
          } else {
            clearTimeout(loopLogs);
            term.clear();
            term.write(generatePrompt());
          }
        }, 90);
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