import { xtermOptions, generatePrompt, opener } from './terminal';
import { fauxLogTimeMessageMap } from './faux_logs';

Terminal.applyAddon(terminado);
Terminal.applyAddon(fit);
Terminal.applyAddon(search);

var term = new Terminal(xtermOptions);
var protocol = (location.protocol === 'https:') ? 'wss://' : 'ws://';
var socketURL = protocol + location.hostname + ((location.port) ? (':' + location.port) : '') + "/ws/";
var sock = new WebSocket(socketURL);

sock.addEventListener('open', function(){
  console.log("[ws]: connection opened")
  term.terminadoAttach(sock);
  term.fit();
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
  term.write(generatePrompt());
})

term.on('key', (key, ev) => {
  if (key.charCodeAt(0) == 13) {
    term.write("\n");
    term.write(key);
  }
})

sock.addEventListener('close', function(event){
  console.log('[ws]: connection closed: ', event);
  term.writeln("");
  term.writeln("connection closed :(");
  term.terminadoDetach(sock);
})

function canParse(c) {
  if (c.length == 1) {
    return false
  }

  try {
    return JSON.parse(c) && true;
  } catch(err) {
    return false
  }
}

sock.addEventListener('message', function(event) {
  console.log("[ws]: message from server: ", event);
  var parsed, responseKey, responseMessage;

  if (canParse(event.data)) {
    parsed = JSON.parse(event.data);
    responseKey = parsed.key;
    responseMessage = parsed.message;
  }

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

window.onresize = function() {
  term.fit();
};