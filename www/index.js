import { Latin } from "puzzle";

const pre = document.getElementById("puzzle-canvas");
const latin = Latin.mynew();
const ortho = latin.orthogonal();

const renderLoop = () => {
    // pre.textContent = latin.render();
    pre.textContent = ortho.render();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
