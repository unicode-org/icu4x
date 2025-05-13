import { RenderInfo, lib } from "../../gen/index.mjs";
import { TerminusRender } from "../../gen/rendering/rendering.mjs";

let params = new URLSearchParams(window.location.search);

let func = params.get("func");

let terminus = new TerminusRender(lib, () => {}, RenderInfo.termini[func]);

document.getElementById("render").appendChild(terminus);