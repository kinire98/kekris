import { invoke } from "@tauri-apps/api/core";
import { getRepeatInterval, getStartRepeatInterval } from "./interval";

export default function manageInputListeners() {
    // ! Take out to another file
    const customRepeatInterval = getRepeatInterval(); // Customize this value (in milliseconds)
    const customStartRepeatInteval = getStartRepeatInterval();
    const keyIntervals: Record<string, NodeJS.Timeout> = {}; // Tracks active intervals for keys
    const keySet = new Set<string>();

    // Handle keydown event
    document.addEventListener("keydown", (event: KeyboardEvent) => {
        if (keyIntervals[event.key]) return;

        // Trigger the action immediately
        manageInput(event.key);

        // Set up a custom interval for repeated actions

        keySet.add(event.key);
        setTimeout(() => {
            if (keySet.has(event.key)) {
                keyIntervals[event.key] = setInterval(() => {
                    manageInput(event.key);
                }, customRepeatInterval);
            }
        }, customStartRepeatInteval);
    });

    // Handle keyup event
    document.addEventListener("keyup", (event: KeyboardEvent) => {
        if (keySet.has(event.key)) keySet.delete(event.key);
        if (keyIntervals[event.key]) {
            clearInterval(keyIntervals[event.key]);
            delete keyIntervals[event.key];
        }
    });
}


function manageInput(keyCode: string) {
    switch (keyCode) {
        case "ArrowDown":
            hardDrop();
            break;
        case "ArrowLeft":
            leftMove();
            break;
        case "ArrowRight":
            rightMove();
            break;
        case "Shift":
            savePiece();
            break;
        case " ":
            softDrop();
            break;
        case "a":
            counterClockWise();
            break;
        case "s":
            fullRotation();
            break;
        case "d":
            clockwise();
            break;
        case "1":
            targetingRandom();
            break;
        case "2":
            targetingEven();
            break;
        case "3":
            targetingEliminations();
            break;
        case "4":
            targetingPayback();
            break;
        case "Escape":
            forfeit();
            break;
        case "p":
            retryGame();
            break;
    }
}

async function clockwise() {
    await invoke("clockwise_rotation");
}

async function counterClockWise() {
    await invoke("counter_clockwise_rotation");
}

async function forfeit() {
    await invoke("forfeit_game")
}

async function fullRotation() {
    await invoke("full_rotation");
}

async function hardDrop() {
    await invoke("hard_drop");
}

async function leftMove() {
    await invoke("left_move");
}

async function retryGame() {
    await invoke("retry_game");
}

async function rightMove() {
    await invoke("right_move");
}

async function savePiece() {
    await invoke("save_piece");
}

async function softDrop() {
    await invoke("soft_drop");
}

async function targetingEliminations() {
    await invoke("targeting_strategy_eliminations");
}

async function targetingEven() {
    await invoke("targeting_strategy_even");
}

async function targetingRandom() {
    await invoke("targeting_strategy_random");
}

async function targetingPayback() {
    await invoke("targeting_strategy_payback");
}
