import init, { animalese, animalese_wav} from "./wasm/animalese_wasm.js";


// import init, {Animalese} from "./wasm/animalese_wasm.js";

// const pitchRange = 1//document.getElementById("pitch");
const inputElement = document.getElementById("input");
const shortenCheckbox = document.getElementById("shorten");
const pitchRange = document.getElementById("pitch");
const submitButton = document.getElementById("submit");
const audioElement = document.getElementById("audio");

submitButton.addEventListener("click", async () => {
  const inputText = inputElement.value;
  const shorten = shortenCheckbox.checked;
  const pitch = parseFloat(pitchRange.value);
  console.log("inputText", inputText)
  const animaleseBuffer = animalese_wav(inputText, shorten, pitch);
  console.log(animaleseBuffer);
  const blob = new Blob([new Uint8Array(animaleseBuffer)], { type: "audio/wav" });
  const url = URL.createObjectURL(blob);
  audioElement.src = url;
  await audioElement.play();
});

init().then(() => {
  console.log("Loaded animalese_wasm");  
});