import { RenderInfo } from "icu4x/demo/index.mjs";

Object.values(RenderInfo.termini).forEach((t) => {
	let a = document.createElement("li");
	a.innerHTML = `<a href="icu4x/demo/${t.html}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});