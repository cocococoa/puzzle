import { Latin } from "puzzle";

const CELL_SIZE = 50; // px
const FONT_SIZE = 40; // px
const GRID_COLOR = "#CCCCCC";

const latin = Latin.mynew();
const ortho = latin.orthogonal();
const transversal_list = latin.transversal();
const size = latin.size();
const latin_canvas = document.getElementById("latin-canvas");
const trans_canvas = document.getElementById("trans-canvas");
const ortho_canvas = document.getElementById("ortho-canvas");
const trans_num_span = document.getElementById("number-of-transversals");
const latin_ctx = latin_canvas.getContext("2d");
const trans_ctx = trans_canvas.getContext("2d");
const ortho_ctx = ortho_canvas.getContext("2d");
latin_canvas.height = (CELL_SIZE + 1) * size + 1;
latin_canvas.width = (CELL_SIZE + 1) * size + 1;
trans_canvas.height = (CELL_SIZE + 1) * size + 1;
trans_canvas.width = (CELL_SIZE + 1) * size + 1;
ortho_canvas.height = (CELL_SIZE + 1) * size + 1;
ortho_canvas.width = (CELL_SIZE + 1) * size + 1;

const renderLoop = () => {
    drawGrid(latin_ctx, latin.size());
    drawLatinCells(latin_ctx, latin);
    trans_num_span.textContent = transversal_list.size().toString();
    drawGrid(trans_ctx, latin.size());
    drawTransCells(trans_ctx, latin, transversal_list.get(30));
    drawGrid(ortho_ctx, ortho.size());
    drawLatinCells(ortho_ctx, ortho);

    requestAnimationFrame(renderLoop);
};

const drawGrid = (ctx, size) => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines
    for (let i = 0; i <= size; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * size + 1);
    }

    // Horizontal lines
    for (let j = 0; j <= size; ++j) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * size + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const drawLatinCells = (ctx, board) => {
    ctx.beginPath();

    ctx.font = FONT_SIZE.toString() + "px Arial";
    ctx.textAlign = "center";
    for (let row = 0; row < board.size(); row++) {
        for (let col = 0; col < board.size(); col++) {
            ctx.fillText(
                board.get(row, col).toString(),
                col * (CELL_SIZE + 1) + (CELL_SIZE + 1) / 2,
                (row + 1) * (CELL_SIZE + 1) - (CELL_SIZE + 1) / 10,
                CELL_SIZE
            )
        }
    }

    ctx.stroke();
};

const drawTransCells = (ctx, board, transversal) => {
    ctx.beginPath();

    ctx.textAlign = "center";
    for (let row = 0; row < board.size(); row++) {
        for (let col = 0; col < board.size(); col++) {
            if (transversal.get(col) == row) {
                ctx.font = FONT_SIZE.toString() + "px Arial";
            } else {
                ctx.font = (FONT_SIZE / 2).toString() + "px Arial";
            }
            ctx.fillText(
                board.get(row, col).toString(),
                col * (CELL_SIZE + 1) + (CELL_SIZE + 1) / 2,
                (row + 1) * (CELL_SIZE + 1) - (CELL_SIZE + 1) / 10,
                CELL_SIZE
            )
        }
    }

    ctx.stroke();
};


requestAnimationFrame(renderLoop);
