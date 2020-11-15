import { A } from './common';

console.log(`hi, did i import ${A} correctly?`);

Terminal.applyAddon(terminado);
Terminal.applyAddon(fit);
Terminal.applyAddon(search);

var term = new Terminal();
var protocol = (location.protocol === 'https:') ? 'wss://' : 'ws://';
var socketURL = protocol + location.hostname + ((location.port) ? (':' + location.port) : '') + "/ws/";
console.log("socket url: ", socketURL)
var sock = new WebSocket(socketURL);

sock.addEventListener('open', function(){
  console.log("open")
  term.terminadoAttach(sock);
  term.fit();
})

sock.addEventListener('close', function(){
  console.log('close');
  term.writeln("");
  term.writeln("connection closed");
  term.terminadoDetach(sock);
})

term.open(document.getElementById('terminal'));
window.onresize = function() {term.fit();};