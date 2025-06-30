function generateTemplate(className, variable, selector) {
    if (className[variable] === undefined) {
        className[variable] = document.querySelector(selector).content;
    }
}

class ParameterTemplate extends HTMLElement {
    default;

    inputElement = null;

    static baseTemplate;
    constructor(options = {}, className, selector) {
        super();
        generateTemplate(ParameterTemplate, "baseTemplate", "#parameter");
        generateTemplate(className, "template", selector);
        let baseClone = ParameterTemplate.baseTemplate.cloneNode(true);

        let clone = className["template"].cloneNode(true);

        this.initialize(clone, options);

        this.inputElement = clone.querySelector("*[data-oninput]");
        if (this.inputElement !== null) {
            this.inputElement.addEventListener("input", this.input.bind(this));
        }

        clone.firstElementChild.slot = "parameter";
        this.appendChild(clone);

        const shadowRoot = this.attachShadow({ mode: "open" });
        shadowRoot.appendChild(baseClone);

        if ("defaultValue" in options) {
            this.default = options.defaultValue;
            this.setValue(this.default);
        }
    }

    setValue(v) {
        if (this.inputElement !== null) {
            this.inputElement.value = v;
        }
    }

    getEventValue(event) {
        return event.target.value;
    }

    getEventExpr(event) {
        return JSON.stringify(event.target.value);
    }

    input(event) {
        this.dispatchEvent(new CustomEvent("parameter-input", {
            detail: { value: this.getEventValue(event), expr: this.getEventExpr(event) }
        }));
    }

    initialize() {}
}

customElements.define("terminus-param", ParameterTemplate);

class BooleanTemplate extends ParameterTemplate {
    static template;
    constructor(options) {
        super(options, BooleanTemplate, "template#boolean");
    }

    getEventValue(event) {
        return event.target.checked;
    }

    setValue(v) {
        this.inputElement.checked = v;
    }

    getEventExpr(event) {
        return event.target.value ? 'true' : 'false';
    }
}

customElements.define("terminus-param-boolean", BooleanTemplate);

class NumberTemplate extends ParameterTemplate {
    static template;
    constructor(options) {
        super(options, NumberTemplate, "template#number");
    }

    getEventValue(event) {
        return parseFloat(event.target.value);
    }

    getEventExpr(event) {
        return event.target.value;
    }
}

customElements.define("terminus-param-number", NumberTemplate);

class StringTemplate extends ParameterTemplate {
    static template;
    constructor(options) {
        super(options, StringTemplate, "template#string");
    }
}

customElements.define("terminus-param-string", StringTemplate);

class CodepointTemplate extends ParameterTemplate {
    static template;
    constructor(options) {
        super(options, CodepointTemplate, "template#string");
    }

    getEventValue(event) {
        let value = event.target.value.replace("U+", "0x").toLowerCase();
        if (value.startsWith("0x") && value.endsWith(parseInt(value, 16).toString(16))) {
            return parseInt(value, 16);
        }
        return value.codePointAt(0);
    }

    getEventExpr(event) {
        let value = event.target.value.replace("U+", "0x").toLowerCase();
        if (value.startsWith("0x") && value.endsWith(parseInt(value, 16).toString(16))) {
            return "0x" + value.toUpperCase().substring(2);
        }
        return `"${value}".codePointAt(0)`;
    }

    setValue(v) {
        this.inputElement.value = String.fromCodePoint(v);
    }
}

customElements.define("terminus-param-codepoint", CodepointTemplate);

class StringArrayTemplate extends ParameterTemplate {
    static template;
    constructor(options) {
        super(options, StringArrayTemplate, "template#string-array");
    }

    getEventValue(event) {
        return event.target.value.split(",").map((s) => s.trim());
    }

    getEventExpr(event) {
        return '[' + event.target.value.split(",").map((s) => s.trim()).map(JSON.stringify) + ']';
    }
}

customElements.define("terminus-param-string-array", StringArrayTemplate);

class EnumOption extends HTMLElement {
    static template;
    constructor(optionText) {
        super();
        generateTemplate(EnumOption, "template", "template#enum-option");
        let clone = EnumOption.template.cloneNode(true);

        clone.querySelector("slot[name='option-text']").parentElement.innerText = optionText;

        this.append(...clone.children);
    }
}

customElements.define("terminus-enum-option", EnumOption);

class EnumTemplate extends ParameterTemplate {
    static template;

    constructor(options) {
        super(options, EnumTemplate, "template#enum");
    }

    initialize(clone, options) {
        let values = clone.querySelector("*[data-options]");

        let nullOption = document.createElement("option");
        nullOption.setAttribute("value", "null");
        nullOption.setAttribute("disabled", "true");
        nullOption.setAttribute("selected", "true");
        nullOption.setAttribute("style", "display:none");
        values.appendChild(nullOption);

        for (let entry of options.values) {
            values.append(...(new EnumOption(entry)).children);
        }
    }

    getEventValue(event) {
        return event.target.value;
    }

    getEventExpr(event) {
        return "'" + event.target.value + "'";
    }
}

customElements.define("terminus-param-enum", EnumTemplate);

class TerminusParams extends HTMLElement {
    #params = [];
    #exprs = [];

    constructor(params, evaluateExternal) {
        super();

        for (var i = 0; i < params.length; i++) {
            let param = params[i];
            let paramName = document.createElement("span");
            paramName.slot = "param-name";
            paramName.innerText = param.name;

            var newChild;

            switch (param.typeUse) {
                case "string":
                    newChild = new StringTemplate(param);
                    break;
                case "boolean":
                    newChild = new BooleanTemplate(param);
                    break;
                case "number":
                    newChild = new NumberTemplate(param);
                    break;
                case "Array<string>":
                    newChild = new StringArrayTemplate(param);
                    break;
                case "enumerator":
                    newChild = new EnumTemplate(param);
                    break;
                case "codepoint":
                    newChild = new CodepointTemplate(param);
                    break;
                case "external":
                    let updateParamEvent = (value) => {
                        this.#params[i] = value;
                    };
                    evaluateExternal(param, updateParamEvent);
                    break;
                default:
                    console.error("Unrecognized parameter: ", param);
                    break;
            }

            newChild.addEventListener("parameter-input", this.input.bind(this, i));
            this.#params[i] = newChild.default;
            this.#exprs[i] = param.name;

            newChild.appendChild(paramName);
            this.appendChild(newChild);
        }
    }

    input(paramIdx, event) {
        this.#params[paramIdx] = event.detail.value;
        this.#exprs[paramIdx] = event.detail.expr;
    }

    get values() {
        return this.#params;
    }

    get exprs() {
        return this.#exprs;
    }
}

customElements.define("terminus-params", TerminusParams);

export class TerminusRender extends HTMLElement {
    static template;

    #func = null;
    #display = null;
    #expr = null;
    #parameters;
    #output;
    #code;
    constructor(terminus, exprCallback = (() => {}), evaluateExternal = (() => {}))  {
        super();
        generateTemplate(TerminusRender, "template", "template#terminus");
        let clone = TerminusRender.template.cloneNode(true);

        this.id = terminus.funcName;

        this.#func = terminus.func;
        this.#display = terminus.display;
        this.#expr = terminus.expr;

        let button = clone.querySelector("*[data-submit]");
        button.addEventListener("click", this.submit.bind(this));
        button.setAttribute("disabled", "true");

        let funcText = document.createElement("span");
        funcText.slot = "func-name";
        funcText.innerText = terminus.funcName;
        this.appendChild(funcText);

        this.#parameters = new TerminusParams(terminus.parameters, evaluateExternal);
        this.#parameters.slot = "parameters";
        this.appendChild(this.#parameters);

        this.#code = document.createElement("code");
        this.#code.slot = "code";
        this.appendChild(this.#code);

        this.#output = document.createElement("span");
        this.#output.slot = "output";
        this.appendChild(this.#output);

        const shadowRoot = this.attachShadow({ mode: "open" });
        shadowRoot.appendChild(clone);

        this.#code.innerText = this.#expr(...this.#parameters.exprs);
        exprCallback(this.#code);
        for (let param of this.#parameters.children) {
            param.addEventListener('parameter-input', () => {
                this.#code.innerText = this.#expr(...this.#parameters.exprs);
                exprCallback(this.#code);                
                this.#output.innerText = "";
                this.#output.classList = "";
                if (this.#parameters.values.every((e) => e != undefined)) {
                    button.removeAttribute("disabled");
                }
            });
        }
    }

    submit() {
        try {
            this.#output.innerText = (this.#display || ((o) => o))(this.#func(...this.#parameters.values));
            this.#output.classList = "";
        } catch(e) {
            this.#output.innerText = e.message;
            this.#output.classList = "error";
            throw e;
        }
    }
}

customElements.define("terminus-render", TerminusRender);