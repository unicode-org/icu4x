// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { RenderInfo, icu } from "../../gen/index.mjs";
import { TerminusRender } from "../../gen/rendering/rendering.mjs";
import beautify from 'js-beautify';
import Prism from 'prismjs';

window.icu = window.top.icu = icu;

// Renders all termini into the class="container" element
Object.values(RenderInfo.termini).toSorted((a, b) => a.funcName < b.funcName ? -1 : 1).forEach((t) => {
    let details = document.createElement("details");
    let summary = document.createElement("summary");
    summary.innerHTML = `<code>${t.funcName}</code>`;
    details.appendChild(summary);
    details.appendChild(document.createElement("br"));
    details.appendChild(new TerminusRender(
        RenderInfo.termini[t.funcName],
        (el) => {
            // Necessary for Prism to know the language to highlight for, and also
            // to ensure CSS `white-space: pre-wrap` is applied from selector
            el.classList.add("language-js");
            el.textContent = beautify.js(el.textContent, {
                indent_size: 2,
                indent_char: " ",
                break_chained_methods: true,
                // brace_style: "collapse",
                wrap_line_length: 45,
            });
            Prism.highlightElement(el);
        },
    ));
    document.getElementsByClassName("container")[0].appendChild(details);
});

document.querySelector("#loading").hidden = true;
document.querySelector("#loaded").hidden = false;

function updateFromHash() {
    const hash = window.location.hash;
    if (!hash || hash.length < 2) return;

    const cleanHash = hash.substring(1);
    const parts = cleanHash.split('?');
    const funcName = parts[0];
    const queryString = parts[1];

    if (!funcName) return;

    const terminusRender = document.getElementById(funcName);
    if (!terminusRender) {
        console.warn(`Function ${funcName} not found`);
        return;
    }

    const details = terminusRender.closest('details');
    if (details) {
        details.open = true;
        details.scrollIntoView();
    }

    // Optimization: If no query string, stop here.
    if (!queryString) return;

    const params = new URLSearchParams(queryString);
    const terminusParams = terminusRender.querySelector('terminus-params');
    if (!terminusParams) return;

    for (const paramElement of terminusParams.children) {
        const nameSlot = paramElement.querySelector('[slot="param-name"]');
        if (!nameSlot) continue;
        const paramName = nameSlot.innerText;

        // Optimization: Check existence and capability before calculating value
        if (params.has(paramName) && paramElement.setValue) {
            const valStr = params.get(paramName);
            const tagName = paramElement.tagName.toLowerCase();

            if (tagName === 'terminus-param-boolean') {
                paramElement.setValue(valStr === 'true' || valStr === '1');
            } else if (tagName === 'terminus-param-number') {
                paramElement.setValue(parseFloat(valStr));
            } else if (tagName === 'terminus-param-codepoint') {
                if (valStr.startsWith('0x') || valStr.startsWith('U+')) {
                    paramElement.setValue(parseInt(valStr.replace('U+', '0x'), 16));
                } else if (/^[0-9]+$/.test(valStr)) {
                    paramElement.setValue(parseInt(valStr, 10));
                } else {
                    paramElement.setValue(valStr.codePointAt(0));
                }
            } else {
                // Default/Enum fallback
                paramElement.setValue(valStr);
            }

            if (paramElement.inputElement) {
                paramElement.inputElement.dispatchEvent(new Event('input', { bubbles: true }));
            }
        }
    }
}

window.addEventListener('hashchange', updateFromHash);
if (window.location.hash) {
    updateFromHash();
}
