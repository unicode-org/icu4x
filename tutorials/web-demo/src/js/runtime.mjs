import { RenderInfo, lib } from "icu4x/demo";
import { TerminusRender } from "icu4x/demo/rendering";

let params = new URLSearchParams(window.location.search);

let func = params.get("func");

const dataProvider = lib.DataProvider.compiled();

let terminus = new TerminusRender(lib, (param, updateParamEvent) => {
    if (parameter.type === "DataProvider") {
        updateParamEvent(dataProvider);
    } else {
        console.error(`Unrecognized parameter type ${param}`);
    }
}, RenderInfo.termini[func]);

document.getElementById("render").appendChild(terminus);