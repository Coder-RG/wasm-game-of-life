import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const emptyCanvasButton = document.getElementById("empty-canvas");

const CELL_SIZE = 5; // px

let universe = Universe.new();
const width = universe.width();
const height = universe.height();

let animationId = null;

pre.height = (CELL_SIZE + 1) * height + 1;
pre.width = (CELL_SIZE + 1) * width + 1;

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    animationId = requestAnimationFrame(renderLoop);
};

const emptyCanvas = () => {
    universe = Universe.empty();
    pre.textContent = universe.render();
    universe.tick();

    animationId = requestAnimationFrame(renderLoop);
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
