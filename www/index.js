import {memory} from "code-musique/code_musique_bg";
import * as wasm from "code-musique";

const bpm = 100;


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
    let bufferSourceNode = audioContext.createBufferSource();
    let buffer = new AudioBuffer({
        length: 44_000 * 60/bpm, 
        numberOfChannels: 1, 
        sampleRate: 44000,
    });
    bufferSourceNode.buffer = buffer;


    let result = wasm.compile(event.target.value);
    let memoire = new Float32Array(memory.buffer, result.pointer, result.size);
    buffer.getChannelData(0).set(memoire);

});

playBtn.addEventListener("click", (event) => {
});