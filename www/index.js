import {memory} from "code-musique/code_musique_bg";
import * as wasm from "code-musique";

const bpm = 100;


const codeArea = document.getElementById("codeArea");
const playBtn = document.getElementById("playBtn");


let audioContext = new AudioContext();

codeArea.addEventListener("input", (event)=> {
    let bufferSourceNode = audioContext.createBufferSource();
    bufferSourceNode.connect(audioContext.destination);
    let buffer = new AudioBuffer({
        length: 44000 * 3, 
        numberOfChannels: 1, 
        sampleRate: 44000,
    });
    let result = wasm.compile(event.target.value);
    let memoire = new Float32Array(memory.buffer, result.pointer, result.size);
    buffer.getChannelData(0).set(memoire);
    bufferSourceNode.buffer = buffer;
    bufferSourceNode.start();
});

playBtn.addEventListener("click", (event) => {
});