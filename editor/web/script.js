let galType = "g22v10";




function get_title() {
    return "OpenGal to JEDEC Compiler"
}

function setup_editor(monaco, editor_el) {
    monaco.languages.register({
        id: "open-gal",
    });

    monaco.languages.setMonarchTokensProvider("open-gal", {
        keywords: ["fill", "count", "dff"],
        typeKeywords: ["pin", "table"],
        operators: ["=", "!", "&", "|", "^", "->"],
        symbols: /[=><!~?:&|+\-*\/\^%]+/,
        escapes:
            /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,
        tokenizer: {
            root: [
                [
                    /[a-z_$][\w$]*/,
                    {
                        cases: {
                            "@typeKeywords": "keyword",
                            "@keywords": "keyword",
                            "@default": "identifier",
                        },
                    },
                ],
                [/[A-Z][\w\$]*/, "type.identifier"],
                { include: "@whitespace" },
                [/[{}()\[\]]/, "@brackets"],
                [/[<>](?!@symbols)/, "@brackets"],
                [/@symbols/, { cases: { "@operators": "operator", "@default": "" } }],
                [/\d+/, "number"],
                [/[;,.]/, "delimiter"],
            ],
            comment: [
                [/[^\/*]+/, "comment"],
                [/\/\*/, "comment", "@push"],
                ["\\*/", "comment", "@pop"],
                [/[\/*]/, "comment"],
            ],
            whitespace: [
                [/[ \t\r\n]+/, "white"],
                [/\/\*/, "comment", "@comment"],
                [/\/\/.*$/, "comment"],
            ],
        },
    });

    const editor = monaco.editor.create(editor_el, {
        value: getCode(),
        language: 'open-gal',
        theme: 'vs-dark'
    });

    return editor;
}


function run_code(editor, output_el) {

    const code = editor.getValue();
    const result = wasm_bindgen.rs_compile(code, galType);
    console.log(result);

    if (result.Err != null) {
        output_el.innerText += result.Err;
    } else {
        download(result.Ok, "open_gal.jed");
        output_el.innerText += "Compiled successfully";
    }

}


function setup_cmd(editor, output_el) {
    const commands = {
        // add commands here
        help: () => "Available commands: help, clear ...",
        compile: (args) => {
            const code = editor.getValue();
            const result = wasm_bindgen.rs_compile(code, galType);

            if (args.length === 0) {
                if (result.Err != null) {
                    output_el.innerText += result.Err;
                } else {
                    output_el.innerText += "Compiled successfully\n";
                    output_el.innerText += result.Ok;
                }
            } else if (args.length === 2 && args[0] == ">") {
                if (result.Err != null) {
                    output_el.innerText += result.Err;
                } else {
                    download(result.Ok, args[1]);
                    output_el.innerText += "Compiled successfully";
                }
            } else {
                output_el.innerText += "ERROR: unrecognized arguments for `compile`"
            }

        },
        transpile: (args) => {
            const code = editor.getValue();
            const result = wasm_bindgen.rs_transpile(code);

            if (args.length === 0) {
                if (result.Err != null) {
                    output_el.innerText += result.Err;
                } else {
                    output_el.innerText += "Transpiled successfully\nWin Cuple code:\n";
                    output_el.innerText += result.Ok;
                }
            } else if (args.length === 2 && args[0] == ">") {
                if (result.Err != null) {
                    output_el.innerText += result.Err;
                } else {
                    download(result.Ok, args[1]);
                    output_el.innerText += "Transpiled successfully";
                }
            } else {
                output_el.innerText += "ERROR: unrecognized arguments for `transpile`"
            }
        },
        tabledata: (args) => {
            const code = editor.getValue();
            const result = wasm_bindgen.rs_tabledata(code);

            if (args.length === 0) {
                if (result.Err != null) {
                    output_el.innerText += result.Err;
                } else {
                    output_el.innerText += JSON.stringify(result.Ok, null, 2);
                }
            } else if (args.length === 2 && args[0] == ">") {
                if (result.Err != null) {
                    output_el.innerText += result.Err;
                } else {
                    download(JSON.stringify(result.Ok, null, 2), args[1]);
                }
            } else {
                output_el.innerText += "ERROR: unrecognized arguments for `tabledata`"
            }
        },
        clear: () => {
            output_el.innerText = "";
            return "";
        },
    };
    return commands;
}


function download(text, fileName) {
    const blob = new Blob([text], { type: "text/plain" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = fileName;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}


function getCode() {
    return `pin 1, 2 = i[0..1];
pin [14..17] = and, or, xor, not;

table(i0, i1 -> and).fill(0) {
    11 1
}

table(i0, i1 -> or).fill(1) {
    00 0
}

table(i0, i1 -> xor).count {
    0
    1
    1
    0
}

table(i0 -> not) {
    01
    10
}


pin 23 = a;
pin 3 = b;
pin 2 = c;

a = (!b | (c));
a.dff;`;
}