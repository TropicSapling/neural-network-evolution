"use strict";

import init, {greet} from '../wasm/neural-network-evolution.js'

let canvas;

window.onload = function init() {
	canvas = document.getElementById("game").getContext("2d");
}

function draw_bg() {
	// Clear canvas
	canvas.clearRect(0, 0, 600, 600);
	
	// Draw background
	canvas.fillStyle = "#eee";
	canvas.fillRect(0, 0, 600, 600);
}

function draw_agent(r, g, b, x, y, size) {
	canvas.fillStyle = `rgb(r, g, b)`;
	canvas.fillRect(x, y, size, size);
}

init().then(() => {
	console.log("Finished loading WebAssembly.")
})
