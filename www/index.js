import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const emptyCanvasButton = document.getElementById("empty-canvas");

const CELL_SIZE = 5; // px

const HEIGHT = 32;
const WIDTH = 64;
let universe = Universe.new(WIDTH, HEIGHT);
const width = universe.width();
const height = universe.height();

let animationId = null;

pre.height = (CELL_SIZE + 1) * height + 1;
pre.width = (CELL_SIZE + 1) * width + 1;

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
};

const renderLoop = () => {
    fps.render();

    pre.textContent = universe.render();
    universe.tick();

    animationId = requestAnimationFrame(renderLoop);
};

const emptyCanvas = () => {
    universe = Universe.empty(WIDTH, HEIGHT);
    pre.textContent = universe.render();
    // universe.tick();
    //
    // animationId = requestAnimationFrame(renderLoop);
};

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
}

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

const isPaused = () => {
    return animationId === null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

emptyCanvasButton.addEventListener("click", event => {
    emptyCanvas()
});

pre.addEventListener("click", event => {
    const boundingRect = pre.getBoundingClientRect();

    const scaleX = pre.width / boundingRect.width;
    const scaleY = pre.height / boundingRect.height;

    const preLeft = (event.clientX - boundingRect.left) * scaleX;
    const preTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(preTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(preLeft / (CELL_SIZE + 1)), width - 1);

    if (event.ctrlKey || event.metaKey) {
        universe.insert_glider(row, col);
    } else {
        universe.toggle_cell(row, col);
    }
    pre.textContent = universe.render();
});

