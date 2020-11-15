import { A } from './common';
import { xtermOptions, generatePrompt } from './terminal';
import { colorize } from './colors';
import { fauxLogTimeMessageMap } from './faux_logs';

console.log("a: ", A);

const opener = [
  "hey, i'm kathleen - it looks like you found my website!",
  "give me a sec to get organized...",
  "i won't take too long, just a few loose ends to tie up...",
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
        setTimeout(function(){
          term.clear();
          sock.send('print_faux_logs');
        }, 3000);
      }
    }, 1000);
  }

  function killIntroLoop(){
    clearTimeout(loopTimeout);
  }

  loopIntro();
})

sock.addEventListener('close', function(event){
  console.log('[ws]: connection closed: ', event);
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

            switch(fauxLogTimeMessageMap[i.toString()] != undefined) {
              case true:
                term.clear();
                term.writeln(fauxLogTimeMessageMap[i.toString()])
              default:
                break
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