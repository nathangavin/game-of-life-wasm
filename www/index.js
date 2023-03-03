import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/game_of_life_wasm_bg";

const CELL_SIZE = 5; // px
const GRID_COLOUR = "#CCCCCC";
const DEAD_COLOUR = "#FFFFFF";
const ALIVE_COLOUR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById('game-of-life-canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const context = canvas.getContext('2d');

const drawGrid = () => {
    context.beginPath();
    context.strokeStyle = GRID_COLOUR;

    for (let i = 0; i <= width; i++) {
        context.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        context.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        context.moveTo(0, j * (CELL_SIZE + 1) + 1);
        context.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    context.stroke();
};

const getIndex = (row, column) => row * width + column;

const drawCells = () => {
    const cells_pointer = universe.cells();
    const cells = new Uint8Array(memory.buffer, cells_pointer, width * height);

    context.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);

            context.fillStyle = 
                cells[index] === Cell.Dead ? DEAD_COLOUR : ALIVE_COLOUR;
            
            context.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    context.stroke();
};


const renderLoop = () => {
    universe.tick();
    drawGrid();
    drawCells();    
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
