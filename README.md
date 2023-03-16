# animalese_wasm
Implementation of villager "animalese" sounds from Animal Crossing. 

Based on Acedio's [animalese.js](https://github.com/Acedio/animalese.js).

I took this as an opportunity to finally try WASM, and it's pretty neat. 
Attempting to build long audio files from [animalese.js](https://github.com/Acedio/animalese.js) is fairly slow and sometimes crashes with long input strings. With my implementation, I've been able to use over 30,000 characters and generate a downloadable file in around 300ms (although this will likely change based on hardware). However, there is some kind of limit that breaks WASM if the text is too long.

Demo here: [https://rinsworth.github.io/animalese_wasm/](https://rinsworth.github.io/animalese_wasm/)

