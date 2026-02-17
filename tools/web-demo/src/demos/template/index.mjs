// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Starter template for new demos. Copy this directory and add an entry
// to registry.mjs to register your demo.

export function setup(container, icu) {
    let pre = document.createElement("pre");
    pre.textContent = "ICU4X loaded — see console for the icu object.";
    container.appendChild(pre);
    console.log("icu", icu);
}

export function teardown() { }
