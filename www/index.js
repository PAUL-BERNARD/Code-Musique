import {memory} from "code-musique/code_musique_bg";
import * as wasm from "code-musique";

const codeArea = document.getElementById("codeArea");
const playBtn = document.getElementById("playBtn");
const outputArea = document.getElementById("outputArea");
const downloadButton = document.getElementById("downloadButton");
const uploadZone = document.getElementById("uploadZone");


const SAMPLING_RATE = 44000;

// Vrai si le code a changé depuis la dernière compilation
let changedCode = true;
// Numéro de la mesure jouée
let barStartTime = 0;


let audioContext = new AudioContext();

codeArea.addEventListener("input", (event)=> {
    changedCode = true;
    parseAndPlay();
});

document.addEventListener("prepareNextBar",(event) => {
    event.startTime;
});


function parseAndPlay(startTime = 0) {
    let bufferSourceNode = audioContext.createBufferSource();
    bufferSourceNode.connect(audioContext.destination);
    let result = wasm.compile(codeArea.value);
    let resultData = new Float32Array(memory.buffer, result.pointer, result.size);
    let buffer = new AudioBuffer({
        length: result.size, 
        numberOfChannels: 1, 
        sampleRate: SAMPLING_RATE,
    });
    buffer.getChannelData(0).set(memoire);
    bufferSourceNode.buffer = buffer;
    bufferSourceNode.start();

    // Plan next bar
    const barDuration = result.size/SAMPLING_RATE;
    const event = new Event("prepareNextBar");
    event.startTime = startTime+barDuration;
    setTimeout(() => document.dispatchEvent(event), barDuration-100);
}

downloadButton.addEventListener("click", (event) => {
    downloadCode();
})

function downloadCode() {
    const code = codeArea.value;
    const date = new Date();
    const fileName = "codeMusique"+date.toLocaleDateString().replaceAll("/","-") + ".xfzd";
    const href = 'data:text/plain;charset=utf-8,' + encodeURIComponent(code);
    downloadMockup.setAttribute('href', href);
    downloadMockup.setAttribute('download', fileName);

    downloadMockup.click();
}

uploadZone.addEventListener("change", (event) => {
    const file = uploadZone.file;
})





