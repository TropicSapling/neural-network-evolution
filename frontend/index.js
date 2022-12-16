"use strict";

import init, {run} from '../wasm/neural-network-evolution.js'

const FPS = 60;

let canvas;
let loop;

window.stopAll = function stopAll() {
	clearInterval(loop);
}

window.onload = function init() {
	canvas = document.getElementById("game").getContext("2d");
}

window.draw_bg = function draw_bg() {
	// Clear canvas
	canvas.clearRect(0, 0, 600, 600);
	
	// Draw background
	canvas.fillStyle = "#eee";
	canvas.fillRect(0, 0, 600, 600);
}

window.draw_agent = function draw_agent(r, g, b, x, y, size) {
	canvas.fillStyle = `rgb(${r}, ${g}, ${b})`;
	canvas.fillRect(x, y, size, size);
}

init().then(() => {
	console.log("Finished loading WebAssembly.");
	console.log(`Running game at ${FPS} FPS.`);
	loop = setInterval(run, 1000/FPS);
})
