import { Cpu } from "chip8-wasm";
import { memory } from "chip8-wasm/chip8_wasm_bg";

const speed = 8
const cpu = Cpu.new()

document.getElementById("demo").addEventListener("click", () => cpu.load_demo())
document.getElementById("pong").addEventListener("click", () => cpu.load_pong())
document.getElementById("reset").addEventListener("click", () => cpu.reset())
document.getElementById("halt").addEventListener("click", () => cpu.halt())

const speaker = document.getElementById("speaker");
let sound = false;
let sound_timer = 0;

const updateSound = () => {
    sound_timer--;
    if (sound && cpu.sound_timer() == 0 && sound_timer <= 0) {
        sound = false;
        speaker.style.display = "none";
    }
    else if(!sound && cpu.sound_timer() > 0) {
        sound = true;
        sound_timer = 10;
        speaker.style.display = "inline";
    }
}

const timerLoop = () => {
    updateSound()
    requestAnimationFrame(timerLoop)
}

const renderLoop = () => {
    for (let i = 0; i < speed; ++i) {
        cpu.step()
    }
    cpu.update_timers()
    drawDisplay()
    requestAnimationFrame(renderLoop)
}

const canvas = document.getElementById("display");
const ctx = canvas.getContext('2d');

const drawDisplay = () => {
    const size = 8;
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
requestAnimationFrame(timerLoop);


