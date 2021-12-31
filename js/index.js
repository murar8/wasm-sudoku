import { PlayResult, SolveResult, Sudoku } from "../pkg/index";
import { memory } from "../pkg/index_bg.wasm";

const MAX_SEED = 2 ** 32 - 1;
const CLUE_QTY = 25;

const timeouts = [];

/**
 * @param {HTMLElement} target
 * @param {string} name
 * @param {number} duration
 */
function addTransientClass(target, name, duration) {
    const key = target.id + name;
    if (timeouts[key]) clearTimeout(timeouts[key]);
    target.classList.add(name);
    const timeout = setTimeout(() => target.classList.remove(name), duration);
    timeouts[key] = timeout;
}

const params = new URLSearchParams(window.location.search);

function generateSeed() {
    const seed = Math.floor(Math.random() * MAX_SEED);
    params.set("seed", seed);
    window.location.search = params;
}

if (!params.has("seed")) generateSeed();

const seed = params.get("seed");
const sudoku = Sudoku.random(seed, CLUE_QTY);
const gridBuffer = new Uint8Array(memory.buffer, sudoku.getGrid(), sudoku.grid_span ** 2);

/**
 * @param {KeyboardEvent} event
 */
function onItemKeydown(event) {
    if (event.key !== "Backspace" && event.key !== "Delete" && event.key.length !== 1) {
        return false;
    }

    const value = ["Backspace", "Delete"].includes(event.key) ? 0 : parseInt(event.key);

    if (isNaN(value) || value < 0 || value > 9) {
        addTransientClass(event.target, "sudoku-item-error", 200);
    } else if (sudoku.play(event.target.index, value) != PlayResult.Ok) {
        addTransientClass(event.target, "sudoku-item-warn", 200);
    } else {
        event.currentTarget.value = value !== 0 ? value : "";
    }

    if (sudoku.isSolved()) {
        // TODO Win
    }

    event.preventDefault();
    return true;
}

/**
 *
 * @param {boolean} initMode
 */
function draw() {
    const grid = document.getElementById("sudoku-grid");
    grid.innerHTML = "";

    for (const [index, value] of gridBuffer.entries()) {
        const item = document.createElement("input");
        item.classList.add("sudoku-item");
        item.index = index;

        if (value) item.value = value;

        if (!sudoku.isMutableCell(index)) {
            item.readOnly = true;
            item.tabIndex = -1;
        }

        item.addEventListener("keydown", onItemKeydown);
        grid.appendChild(item);
    }
}

function onGenerate() {
    generateSeed();
}

/**
 * @param {MouseEvent} event
 */
function onSolve(event) {
    const result = sudoku.solve();

    // TODO load

    if (result === SolveResult.Solved) {
        draw();
    } else {
        addTransientClass(event.currentTarget, "button-shake", 600);
    }
}

function onReset() {
    sudoku.reset();
    draw();
}

const generateButton = document.getElementById("generate-button");
generateButton.addEventListener("click", onGenerate);
generateButton.classList.remove("hidden");

const solveButton = document.getElementById("solve-button");
solveButton.addEventListener("click", onSolve);
solveButton.classList.remove("hidden");

const resetButton = document.getElementById("reset-button");
resetButton.addEventListener("click", onReset);
resetButton.classList.remove("hidden");

const grid = document.getElementById("sudoku-grid");
grid.style.gridTemplateColumns = `repeat(${sudoku.grid_span}, 1fr)`;
grid.style.gridTemplateRows = grid.style.gridTemplateColumns;

draw();
