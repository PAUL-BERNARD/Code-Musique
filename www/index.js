import {memory} from "code-musique/code_musique_bg";
import * as wasm from "code-musique";

const codeArea = document.getElementById("codeArea");
const playBtn = document.getElementById("playBtn");
const outputArea = document.getElementById("outputArea");
const downloadButton = document.getElementById("downloadButton");
const uploadZone = document.getElementById("uploadZone");
const stopButton = document.getElementById("stopButton");

const PREVISION_MS = 200;
const SAMPLING_RATE = 44000;

// Vrai si le code a changé depuis la dernière compilation
let isPlaying = false;
// Temps de la mesure jouée
let barStartTime = 0;


let audioContext = new AudioContext();

codeArea.addEventListener("input", (event)=> {
    if (!isPlaying) {
        isPlaying = true;
        parseAndPlay();
    }
});

function parseAndPlay() {
    let bufferSourceNode = parse();
    bufferSourceNode.start(barStartTime);
}

document.addEventListener("prepareNextBar",(event) => {
    event.startTime;
    parseAndPlay();
});


function parse() {
    let bufferSourceNode = audioContext.createBufferSource();
    bufferSourceNode.connect(audioContext.destination);
    let result = wasm.compile(codeArea.value);
    let resultData;
    try {
        resultData = new Float32Array(memory.buffer, result.pointer, result.size);
    }
    catch(error) {
        // TODO
    }
    let buffer = new AudioBuffer({
        length: result.size, 
        numberOfChannels: 1, 
        sampleRate: SAMPLING_RATE,
    });
    buffer.getChannelData(0).set(resultData);
    bufferSourceNode.buffer = buffer;
    

    // Plan next bar
    const barDuration = result.size/SAMPLING_RATE;
    const event = new Event("prepareNextBar");
    barStartTime = barStartTime+barDuration;
    setTimeout(() => document.dispatchEvent(event), 1000*barDuration-PREVISION_MS);

    return bufferSourceNode;
}










downloadButton.addEventListener("click", (event) => {
    downloadCode();
})

function downloadCode() {
    const code = codeArea.value;
    const date = new Date();
    const fileName = "codeMusique"+
        date.toLocaleDateString().replaceAll("/","-")+"-"+
        date.toLocaleTimeString().replaceAll(":","")+
        ".xfzd";
    const href = 'data:text/plain;charset=utf-8,' + encodeURIComponent(code);
    downloadMockup.setAttribute('href', href);
    downloadMockup.setAttribute('download', fileName);

    downloadMockup.click();
}

uploadZone.addEventListener("change", (event) => {
    const [file] = uploadZone.files;
    fileReader.readAsText(file);
})

const fileReader = new FileReader();
fileReader.addEventListener("load", () => {
    codeArea.value = fileReader.result;
});




