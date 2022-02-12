import {memory} from "code-musique/code_musique_bg";
import * as wasm from "code-musique";

const codeArea = document.getElementById("codeArea");
//const volumeSlider = document.getElementById("volumeSlider");
let audioContext = new AudioContext();

// volumeSlider.addEventListener("change", (event) => {
//     const newVolumeValue = event.target.valueAsNumber;
//     console.log(newVolumeValue);
// })
let a = 3;
codeArea.addEventListener("keyup", (event)=> {
    console.log(event.target.value);
    let time0 = Date.now();
    let result = wasm.compile(event.target.value);
    console.log(Date.now() - time0);
    let memoire = new Float64Array(memory.buffer, result.pointer, result.size);
    
    console.log(memoire);
})