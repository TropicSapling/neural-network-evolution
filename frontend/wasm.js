import init, {greet} from '../wasm/neural-network-evolution.js'

init().then(() => {
	greet("WebAssembly")
})
