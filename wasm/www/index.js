import { Cpu } from "chip8-wasm";
import { memory } from "chip8-wasm/chip8_wasm_bg";

const speed = 8
const cpu = Cpu.new()

//cpu.load()

const play = document.getElementById("load")
const reset = document.getElementById("reset")
const halt = document.getElementById("halt")

play.addEventListener("click", () => cpu.load())
reset.addEventListener("click", () => cpu.reset())
halt.addEventListener("click", () => cpu.halt())

const renderLoop = () => {
    for (let i = 0; i < speed; ++i)
        cpu.step()
    drawDisplay()
    requestAnimationFrame(renderLoop)
}

const canvas = document.getElementById("display");
const ctx = canvas.getContext('2d');

const size = 8;

const drawDisplay = () => {
    const displayPtr = cpu.display()
    const display = new Uint8Array(memory.buffer, displayPtr, 32 * 64);
    for (let row = 0; row < 32; ++row) {
        for (let col = 0; col < 64; ++col) {
            ctx.fillStyle = display[row*64+col] == 255 ? "#FFFFFF" : "#000000"
            ctx.fillRect(size * col, size * row, size, size)
        }
    }
}

requestAnimationFrame(renderLoop);


