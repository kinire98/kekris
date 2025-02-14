import { invoke } from "@tauri-apps/api/core";

import { drawBoard } from "../board/board";

export default function manageInput(keyCode: string) {
    switch(keyCode) {
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
    await invoke("clockwise_rotation").then((boardState) => drawBoard(boardState as string));
}

async function counterClockWise() {
    await invoke("counter_clockwise_rotation").then((boardState) => drawBoard(boardState as string));
}

async function forfeit() {
    await invoke("forfeit_game").then((boardState) => {
        drawBoard(boardState as string)
    });
}

async function fullRotation() {
    await invoke("full_rotation").then((boardState) => drawBoard(boardState as string));
}

async function hardDrop() {
    await invoke("hard_drop").then((boardState) => drawBoard(boardState as string));
}

async function leftMove() {
    await invoke("left_move").then((boardState) => drawBoard(boardState as string));
}

async function retryGame() {
    await invoke("retry_game").then((boardState) => drawBoard(boardState as string));
}

async function rightMove() {
    console.log("right")
    await invoke("right_move").then((boardState) => drawBoard(boardState as string));
}

async function savePiece() {
    await invoke("save_piece").then((boardState) => drawBoard(boardState as string));
}

async function softDrop() {
    await invoke("soft_drop").then((boardState) => drawBoard(boardState as string));
}

async function targetingEliminations() {
    await invoke("targeting_strategy_eliminations").then((boardState) => drawBoard(boardState as string));
}

async function targetingEven() {
    await invoke("targeting_strategy_even").then((boardState) => drawBoard(boardState as string));
}

async function targetingRandom() {
    await invoke("targeting_strategy_random").then((boardState) => drawBoard(boardState as string));
}

async function targetingPayback() {
    await invoke("targeting_strategy_payback").then((boardState) => drawBoard(boardState as string));
}