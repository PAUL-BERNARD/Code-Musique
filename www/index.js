import {memory} from "code-musique/code_musique_bg";
import * as wasm from "code-musique";

const codeArea = document.getElementById("codeArea");
const playBtn = document.getElementById("playBtn");

let audioContext = new AudioContext();
let bufferSourceNode = new AudioBufferSourceNode(audioContext);
bufferSourceNode.connect(audioContext.destination);


let audioBuffer = new AudioBuffer({
    length:3*44000,
    numberOfChannels:1,
    sampleRate:44000,  
});
bufferSourceNode.buffer = audioBuffer;

codeArea.addEventListener("input", (event)=> {
    console.log(event.target.value);
    let time0 = Date.now();
    let result = wasm.compile(event.target.value);
    console.log(Date.now() - time0);
    let memoire = new Float32Array(memory.buffer, result.pointer, result.size);
    audioBuffer.copyToChannel(memoire,0);

});

playBtn.addEventListener("click", (event) => {
    bufferSourceNode.start();
});