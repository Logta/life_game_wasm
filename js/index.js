import init, { GameOfLife } from "../pkg/rust_webpack_template.js";

async function run() {
    await init();
    

    const canvas = document.getElementById("life-canvas");
    const ctx = canvas.getContext("2d");
    const playPauseBtn = document.getElementById("play-pause-btn");
    const stepBtn = document.getElementById("step-btn");
    const clearBtn = document.getElementById("clear-btn");
    const randomBtn = document.getElementById("random-btn");
    const speedSlider = document.getElementById("speed-slider");
    const speedValue = document.getElementById("speed-value");
    const generationCounter = document.getElementById("generation-counter");

    const CELL_SIZE = 10;
    const width = Math.floor(canvas.width / CELL_SIZE);
    const height = Math.floor(canvas.height / CELL_SIZE);

    let game = new GameOfLife(width, height);
    let isPlaying = false;
    let animationId = null;
    let fps = parseInt(speedSlider.value);

    function updateGenerationCounter() {
        generationCounter.textContent = `Generation: ${game.generation()}`;
    }

    function render() {
        game.render(ctx);
        updateGenerationCounter();
    }

    function tick() {
        game.tick();
        render();
    }

    function gameLoop() {
        if (isPlaying) {
            tick();
            setTimeout(() => {
                animationId = requestAnimationFrame(gameLoop);
            }, 1000 / fps);
        }
    }

    function play() {
        isPlaying = true;
        playPauseBtn.textContent = "Pause";
        stepBtn.disabled = true;
        gameLoop();
    }

    function pause() {
        isPlaying = false;
        playPauseBtn.textContent = "Play";
        stepBtn.disabled = false;
        if (animationId) {
            cancelAnimationFrame(animationId);
        }
    }

    function togglePlayPause() {
        if (isPlaying) {
            pause();
        } else {
            play();
        }
    }

    function step() {
        if (!isPlaying) {
            tick();
        }
    }

    function clear() {
        if (isPlaying) {
            pause();
        }
        game.clear();
        render();
    }

    function randomize() {
        if (isPlaying) {
            pause();
        }
        game.randomize();
        render();
    }

    function getClickCoordinates(event) {
        const rect = canvas.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        const col = Math.floor(x / CELL_SIZE);
        const row = Math.floor(y / CELL_SIZE);
        return { row, col };
    }

    canvas.addEventListener("click", (event) => {
        const { row, col } = getClickCoordinates(event);
        if (row >= 0 && row < height && col >= 0 && col < width) {
            game.toggle_cell(row, col);
            render();
        }
    });

    playPauseBtn.addEventListener("click", togglePlayPause);
    stepBtn.addEventListener("click", step);
    clearBtn.addEventListener("click", clear);
    randomBtn.addEventListener("click", randomize);

    speedSlider.addEventListener("input", (event) => {
        fps = parseInt(event.target.value);
        speedValue.textContent = `${fps} FPS`;
    });

    render();
}

run().catch(console.error);
