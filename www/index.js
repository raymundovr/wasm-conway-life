import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

let animationId = null;
let currentTicks = 0;
let pressedKey = '';

const playPauseButton = document.getElementById('play-pause');
const resetRandomButton = document.getElementById('reset-random');
const resetDeadButton = document.getElementById('reset-dead');
const ticksPerAnimationRange = document.getElementById('ticks-per-animation');
const ticksPerAnimationValue = document.getElementById('ticks-per-animation-value');

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
let universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  if (currentTicks > ticksPerAnimationRange.value) {    
    pause();
    return;
  }
  currentTicks++;

  drawGrid();
  drawCells();

  universe.tick();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
}

const play = () => {  
  playPauseButton.textContent = "⏸";
  renderLoop();
}

const pause = () => {
  currentTicks = 0;
  playPauseButton.textContent = "▶";  
  cancelAnimationFrame(animationId);
  animationId = null;  
}

playPauseButton.addEventListener('click', event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

resetRandomButton.addEventListener('click', _ => {
  universe = Universe.new();
  drawCells();
});

resetDeadButton.addEventListener('click', _ => {
  universe = universe.all_dead();
  drawCells();
});

ticksPerAnimationRange.addEventListener('input', _ => {  
  setTicksPerAnimationValue();
});

document.addEventListener('keydown', event => {
  pressedKey = event.key;
});

document.addEventListener('keyup', _ => {
  pressedKey = '';
});

const setTicksPerAnimationValue = () => {
  ticksPerAnimationValue.innerText = ticksPerAnimationRange.value;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
};
const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n%8);
  return (arr[byte] & mask) === mask;
}
  
const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);
  
        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;
        // ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;
  
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  
    ctx.stroke();
};

canvas.addEventListener('click', event => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  if (pressedKey === 'Control') {
    console.log("Control action");
    universe.draw_glider(row, col);
  } else if (pressedKey === 'Shift') {
    console.log("Shift Action");
  } else {
    universe.toggle_cell(row, col);
  }
  drawGrid();
  drawCells();
});

setTicksPerAnimationValue();
drawGrid();
drawCells();
play();