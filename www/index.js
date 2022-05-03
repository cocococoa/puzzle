import { Latin } from "puzzle";

const latin_canvas = document.getElementById("latin-canvas");
const ortho_canvas = document.getElementById("ortho-canvas");
const latin = Latin.mynew();
const ortho = latin.orthogonal();

const renderLoop = () => {
    latin_canvas.textContent = latin.render();
    ortho_canvas.textContent = ortho.render();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
