import { RenderInfo } from "../../gen/index.mjs";

Object.values(RenderInfo.termini).forEach((t) => {
	let a = document.createElement("a");
	a.classList.add("list-group-item", "list-group-item-action");
	a.innerHTML = `<a href="template.html?func=${t.funcName}" class="text-break">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});