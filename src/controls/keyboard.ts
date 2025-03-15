import { invoke } from "@tauri-apps/api/core";

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
     invoke("clockwise_rotation");
}

async function counterClockWise() {
     invoke("counter_clockwise_rotation");
}

async function forfeit() {
     invoke("forfeit_game")
}

async function fullRotation() {
     invoke("full_rotation");
}

async function hardDrop() {
     invoke("hard_drop");
}

async function leftMove() {
     invoke("left_move");
}

async function retryGame() {
     invoke("retry_game");
}

async function rightMove() {
     invoke("right_move");
}

async function savePiece() {
     invoke("save_piece");
}

async function softDrop() {
     invoke("soft_drop");
}

async function targetingEliminations() {
     invoke("targeting_strategy_eliminations");
}

async function targetingEven() {
     invoke("targeting_strategy_even");
}

async function targetingRandom() {
     invoke("targeting_strategy_random");
}

async function targetingPayback() {
     invoke("targeting_strategy_payback");
}