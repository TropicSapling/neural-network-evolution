"use strict";

let canvas;

// Error handler
window.onerror = function(msg, url, line, column, error) {
	alert("An error has occurred. Check console for details.");
	
	if(error) {
		console.log(`[!] ${msg} in file ${url}`);
		console.log(`Line: ${line}, column: ${column}`);
		console.log("Stack Trace:");
		console.log(error.stack);
	} else {
		alert(`[!] Error: ${msg} in file ${url}\n\nLine: ${line}, column: ${column}`);
	}
	
	stop_neural_networks();
	stop_game();
}

window.onload = function init() {
	canvas = document.getElementById("game").getContext("2d");
}

function drawFrame(AIs) {
	// Clear canvas
	canvas.clearRect(0, 0, 600, 600);
	
	// Draw background
	canvas.fillStyle = "#eee";
	canvas.fillRect(0, 0, 600, 600);
	
	// Draw AIs
	for(const AI of AIs) {
		canvas.fillStyle = `rgb(${AI.colour.r}, ${AI.colour.g}, ${AI.colour.b})`;
		canvas.fillRect(AI.pos.x, AI.pos.y, AI.size, AI.size);
	}
}
