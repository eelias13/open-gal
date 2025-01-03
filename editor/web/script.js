let code = getOpenGal();
let theme = "vs-dark";
const { rs_compile, rs_transpile, rs_tabledata } = wasm_bindgen;
let editor;
let galtype = "g22v10";
let filename = "out";

function setLanguages(monaco) {
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
}

function setUpEditor(container) {
  getCode();
  require.config({ paths: { vs: "vs/" } });
  require(["vs/editor/editor.main"], function () {
    setLanguages(monaco);
    editor = monaco.editor.create(container, {
      theme: theme,
      value: code,
      language: "open-gal",
    });
  });
}

function getOpenGal() {
  return [
    "pin 1, 2 = i[0..1];",
    "pin [14..17] = and, or, xor, not;",
    "",
    "table(i0, i1 -> and).fill(0) {",
    "    11 1",
    "}",
    "",
    "table(i0, i1 -> or).fill(1) {",
    "    00 0",
    "}",
    "",
    "table(i0, i1 -> xor).count {",
    "    0",
    "    1",
    "    1",
    "    0",
    "}",
    "",
    "table(i0 -> not) {",
    "    01",
    "    10",
    "}",
    "",
    "",
    "pin 23 = a;",
    "pin 3 = b;",
    "pin 2 = c;",
    "",
    "a = (!b | (c));",
    "a.dff;",
  ].join("\n");
}

function download(filename, text) {
  var element = document.createElement("a");
  element.setAttribute(
    "href",
    "data:text/plain;charset=utf-8," + encodeURIComponent(text)
  );
  element.setAttribute("download", filename);
  element.style.display = "none";
  document.body.appendChild(element);
  element.click();
  document.body.removeChild(element);
}

function getCode() {
  if (editor != undefined) code = editor.getValue();
}

async function main() {
  await wasm_bindgen("./rust_bg.wasm");

  setUpEditor(document.getElementById("container"));

  document.getElementById("compile").onclick = () => {
    getCode();
    let result = rs_compile(code, galtype);
    if (result.Err != null) {
      document.getElementById("error").innerHTML = result.Err;
    } else {
      download(filename + ".jed", result.Ok);
    }
  };
  document.getElementById("transpile").onclick = () => {
    getCode();
    let result = rs_transpile(code);
    if (result.Err != null) {
      document.getElementById("error").innerHTML = result.Err;
    } else {
      download(filename + ".PLD", result.Ok);
    }
  };
  document.getElementById("tabledata").onclick = () => {
    getCode();
    let result = rs_tabledata(code);
    if (result.Err != null) {
      document.getElementById("error").innerHTML = result.Err;
    } else {
      download(filename + ".json", JSON.stringify(result.Ok));
    }
  };

  document.getElementById("downloadCode").onclick = () => {
    getCode();
    download(filename + ".ogal", code);
  };

  document.getElementById("galtype").onchange = () => {
    galtype = document.getElementById("galtype").value;
  };

  document.getElementById("theme").onchange = () => {
    theme = document.getElementById("theme").value;
    getCode();
    document.getElementById("container").innerHTML = null;
    setUpEditor(document.getElementById("container"));
  };
}
