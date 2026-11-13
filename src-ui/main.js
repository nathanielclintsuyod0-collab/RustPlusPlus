import { invoke } from "@tauri-apps/api/core";
import * as monaco from "monaco-editor";
import EditorWorker from "../node_modules/monaco-editor/esm/vs/editor/editor.worker.js?worker";
import "./style.css";

globalThis.MonacoEnvironment = {
  getWorker() { return new EditorWorker(); }
};

const $ = (s) => document.querySelector(s);
const languages = ["cpp", "rust", "python", "javascript", "java", "csharp"];
let currentPath = "";
let isTauri = !!(window.__TAURI_INTERNALS__);

$("#app").innerHTML = `
<div class="shell">
  <aside class="sidebar"><div class="brand"><div class="brand-mark">R<span>++</span></div><div><b>Rust++</b><small>OFFLINE CODE ASSISTANT</small></div></div>
    <div class="side-label">WORKSPACE</div><button class="nav active" data-tab="editor">▣ <span>Code editor</span></button><button class="nav" data-tab="assistant">✦ <span>Assistant</span></button><button class="nav" data-tab="about">ⓘ <span>About Rust++</span></button>
    <div class="sidebar-bottom"><span class="status-dot"></span><span>Deterministic engine</span><small>No AI APIs</small></div>
  </aside>
  <main class="main"><header class="topbar"><button id="menu" class="icon-btn" aria-label="Menu">☰</button><div><b id="page-title">Code editor</b><small id="platform-label">Local workspace</small></div><div class="top-spacer"></div><span class="offline-pill"><i></i> OFFLINE ENGINE</span></header>
  <section id="editor-view" class="view">
    <div class="welcome"><div><p class="eyebrow">YOUR CODING WORKSPACE</p><h1>Build something<br/><em>brilliant.</em></h1><p class="muted">Create, inspect and improve code with deterministic tools — no generative AI.</p></div><div class="hero-orbit"><div>R<span>++</span></div></div></div>
    <div class="toolbar"><label>LANGUAGE<select id="language">${languages.map(x=>`<option value="${x}">${({cpp:"C++",rust:"Rust",python:"Python",javascript:"JavaScript",java:"Java",csharp:"C#"})[x]}</option>`).join("")}</select></label><div class="tool-actions"><button id="template" class="btn primary">＋ Template</button><button id="explain" class="btn">Explain</button><button id="check" class="btn">Check code</button><button id="run" class="btn primary">▶ Run</button></div></div>
    <div class="workspace"><div class="editor-card"><div class="card-head"><span><i class="file-dot"></i><span id="filename">untitled.cpp</span></span><div><button id="open" class="tiny">Open</button><button id="save" class="tiny">Save</button></div></div><div id="editor" class="monaco-editor-host" aria-label="Source code editor"></div><div class="editor-foot"><span id="line-count">1 line</span><span>LOCAL FILE</span></div></div><div class="result-card"><div class="card-head"><b>Rust++ results</b><span class="result-tag">BUILT-IN RULES</span></div><div id="result" class="result-body"><div class="empty-state"><div class="empty-icon">✦</div><b>Ready when you are</b><p>Generate a starter, explain a language, or check your source code.</p></div></div></div></div>
  </section>
  <section id="assistant-view" class="view hidden">
    <div class="section-intro">
      <p class="eyebrow">RULE-BASED ASSISTANT</p>
      <h1>Ask Rust++.</h1>
      <p class="muted">Choose a common programming question or enter your own. Examples follow your selected language.</p>
    </div>
    <div class="common-questions">
      <div class="card-head"><b>Common questions</b><span class="result-tag">12 TOPICS</span></div>
      <div id="common-question-list" class="question-grid"></div>
    </div>
    <div class="ask-card">
      <textarea id="question" rows="4" placeholder="Example: What does undefined reference mean?"></textarea>
      <button id="ask" class="btn primary">Analyze request →</button>
    </div>
    <div id="assistant-result" class="result-card assistant-output">
      <div class="empty-state"><div class="empty-icon">✦</div><b>No analysis yet</b><p>Your results will appear here.</p></div>
    </div>
  </section>
  <section id="about-view" class="view hidden"><div class="section-intro"><p class="eyebrow">ABOUT THE PROJECT</p><h1>Built on rules.<br/><em>Powered by Rust.</em></h1><p class="muted">Rust++ is a deterministic coding assistant. It does not call AI services or run a generative model.</p></div><div class="about-grid"><div class="about-tile"><b>6</b><span>Supported languages</span></div><div class="about-tile"><b>0</b><span>AI APIs and models</span></div><div class="about-tile"><b>Local</b><span>Core processing</span></div></div><p class="muted note">Validation is lightweight structural checking, not a substitute for each language's compiler or interpreter.</p></section>
  <footer><span>Rust++ <span class="muted">· Cross-platform foundation</span></span><span id="runtime">Web preview</span></footer>
</main></div>`;


const languageIds = {
  cpp: "cpp",
  rust: "rust",
  python: "python",
  javascript: "javascript",
  java: "java",
  csharp: "csharp"
};

const editor = monaco.editor.create($("#editor"), {
  value: "// Welcome to Rust++\n// Start writing your code here.",
  language: languageIds[$("#language").value],
  theme: "vs-dark",
  automaticLayout: true,
  fontFamily: "'DM Mono', monospace",
  fontSize: 12,
  lineHeight: 22,
  tabSize: 4,
  insertSpaces: true,
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  roundedSelection: false,
  renderLineHighlight: "all",
  padding: { top: 16, bottom: 16 },
  wordWrap: "on",
  smoothScrolling: true,
  cursorBlinking: "smooth"
});

function getSource() {
  return editor.getValue();
}

function setSource(value) {
  editor.setValue(value);
  updateLines();
}

function showResult(target, reply) {
  const el = $(target);
  const code = reply.code ? `<pre><code>${escapeHtml(reply.code)}</code></pre>` : "";
  const list = (reply.explanation || []).map(x=>`<li>${escapeHtml(x)}</li>`).join("");
  el.innerHTML = `<div class="reply"><div class="reply-kicker">${escapeHtml(reply.source || "RUST++")}</div><h2>${escapeHtml(reply.title)}</h2><p>${escapeHtml(reply.message)}</p>${code}${list?`<h3>What this means</h3><ul>${list}</ul>`:""}</div>`;
}
function escapeHtml(s="") { return String(s).replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c])); }
function updateLines(){const n=editor.getModel()?.getLineCount() || 1;$("#line-count").textContent=`${n} ${n===1?"line":"lines"}`;}
function extension(){return ({cpp:"cpp",rust:"rs",python:"py",javascript:"js",java:"java",csharp:"cs"})[$("#language").value];}


const browserTemplates = {
  cpp: `#include <iostream>

int main() {
    std::cout << "Hello, Rust++!" << std::endl;
    return 0;
}`,
  rust: `fn main() {
    println!("Hello, Rust++!");
}`,
  python: `def main():
    print("Hello, Rust++!")

if __name__ == "__main__":
    main()`,
  javascript: `function main() {
    console.log("Hello, Rust++!");
}

main();`,
  java: `public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, Rust++!");
    }
}`,
  csharp: `using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, Rust++!");
    }
}`
};

const browserLanguageInfo = {
  cpp: ["C++", "A compiled general-purpose programming language.", "Supports functions, classes, templates, and manual memory management."],
  rust: ["Rust", "A compiled systems programming language.", "Uses ownership and borrowing to help ensure memory safety."],
  python: ["Python", "A high-level programming language.", "Uses indentation and dynamic typing."],
  javascript: ["JavaScript", "A programming language used for web and application development.", "Supports objects, functions, and asynchronous programming."],
  java: ["Java", "A class-based programming language.", "Runs on the Java Virtual Machine."],
  csharp: ["C#", "A modern programming language for the .NET platform.", "Supports object-oriented and strongly typed programming."]
};

function browserCommand(command, args = {}) {
  if (command === "generate_template") {
    const code = browserTemplates[args.language];
    if (!code) throw new Error("Unsupported language.");

    return {
      source: "RUST++ TEMPLATE ENGINE",
      title: "Starter template generated",
      message: "A built-in starter template was generated locally.",
      code,
      explanation: [
        "This template is a starting point.",
        "You can edit it directly in the code editor."
      ]
    };
  }

  if (command === "explain_language") {
    const info = browserLanguageInfo[args.language];
    if (!info) throw new Error("Unsupported language.");

    return {
      source: "RUST++ LANGUAGE NOTES",
      title: info[0],
      message: info[1],
      explanation: [info[2]]
    };
  }

  if (command === "validate_source") {
    const source = String(args.source || "");
    const lines = source.split(/\r?\n/);
    const issues = [];

    if (!source.trim()) {
      issues.push("The source file is empty.");
    }

    const pairs = { ")": "(", "]": "[", "}": "{" };
    const stack = [];

    for (const ch of source) {
      if ("([{".includes(ch)) stack.push(ch);

      if (")]}".includes(ch)) {
        if (stack.pop() !== pairs[ch]) {
          issues.push(`Unexpected or unmatched closing delimiter: ${ch}`);
          break;
        }
      }
    }

    if (stack.length) {
      issues.push("One or more opening brackets or braces may be unclosed.");
    }

    const trailing = lines.filter(line => /\s+$/.test(line) && line.trim()).length;

    if (trailing) {
      issues.push(`${trailing} line(s) contain trailing whitespace.`);
    }

    return {
      source: "RUST++ BASIC CHECKS",
      title: issues.length ? "Review suggested" : "Basic checks passed",
      message: issues.length
        ? "The built-in checks found items worth reviewing."
        : `No issues were found by the basic checks. Analyzed ${lines.length} line(s).`,
      explanation: issues.length
        ? issues
        : ["These are lightweight structural checks, not a full compiler or interpreter."]
    };
  }


  if (command === "answer_common_question") {
    const q = String(args.question || "").toLowerCase();
    const lang = args.language;
    const topics = [
      ["first", /first program|hello world|getting started/],
      ["print", /print|output|display text/],
      ["variable", /variable|declare/],
      ["input", /user input|input/],
      ["condition", /if.?else|condition/],
      ["loop", /loop|repeat/],
      ["function", /function|method/],
      ["array", /array|list|collection/],
      ["string", /string|text/],
      ["debug", /debug|troubleshoot/],
      ["comment", /comment/],
      ["return", /return a value|return value/]
    ];
    const topic = topics.find(([, pattern]) => pattern.test(q));
    if (!topic) throw new Error("Choose one of the common questions.");

    const examples = {
      first: {
        cpp: '#include <iostream>\nint main() {\n    std::cout << "Hello, world!";\n    return 0;\n}',
        rust: 'fn main() {\n    println!("Hello, world!");\n}',
        python: 'print("Hello, world!")',
        javascript: 'console.log("Hello, world!");',
        java: 'class Main {\n    public static void main(String[] args) {\n        System.out.println("Hello, world!");\n    }\n}',
        csharp: 'using System;\nclass Program {\n    static void Main() {\n        Console.WriteLine("Hello, world!");\n    }\n}'
      },
      print: {
        cpp: 'std::cout << "Hello";',
        rust: 'println!("Hello");',
        python: 'print("Hello")',
        javascript: 'console.log("Hello");',
        java: 'System.out.println("Hello");',
        csharp: 'Console.WriteLine("Hello");'
      },
      variable: {
        cpp: 'int age = 18;\nstd::string name = "Alex";',
        rust: 'let age = 18;\nlet name = "Alex";',
        python: 'age = 18\nname = "Alex"',
        javascript: 'let age = 18;\nconst name = "Alex";',
        java: 'int age = 18;\nString name = "Alex";',
        csharp: 'int age = 18;\nstring name = "Alex";'
      },
      input: {
        cpp: 'int age;\nstd::cin >> age;',
        rust: 'use std::io;\nlet mut input = String::new();\nio::stdin().read_line(&mut input).unwrap();',
        python: 'name = input("Enter your name: ")',
        javascript: 'const name = prompt("Enter your name:");',
        java: 'Scanner scanner = new Scanner(System.in);\nString name = scanner.nextLine();',
        csharp: 'string name = Console.ReadLine();'
      },
      condition: {
        cpp: 'if (age >= 18) {\n    std::cout << "Adult";\n} else {\n    std::cout << "Minor";\n}',
        rust: 'if age >= 18 {\n    println!("Adult");\n} else {\n    println!("Minor");\n}',
        python: 'if age >= 18:\n    print("Adult")\nelse:\n    print("Minor")',
        javascript: 'if (age >= 18) {\n    console.log("Adult");\n} else {\n    console.log("Minor");\n}',
        java: 'if (age >= 18) {\n    System.out.println("Adult");\n} else {\n    System.out.println("Minor");\n}',
        csharp: 'if (age >= 18) {\n    Console.WriteLine("Adult");\n} else {\n    Console.WriteLine("Minor");\n}'
      },
      loop: {
        cpp: 'for (int i = 0; i < 5; i++) {\n    std::cout << i;\n}',
        rust: 'for i in 0..5 {\n    println!("{i}");\n}',
        python: 'for i in range(5):\n    print(i)',
        javascript: 'for (let i = 0; i < 5; i++) {\n    console.log(i);\n}',
        java: 'for (int i = 0; i < 5; i++) {\n    System.out.println(i);\n}',
        csharp: 'for (int i = 0; i < 5; i++) {\n    Console.WriteLine(i);\n}'
      },
      function: {
        cpp: 'int add(int a, int b) {\n    return a + b;\n}',
        rust: 'fn add(a: i32, b: i32) -> i32 {\n    a + b\n}',
        python: 'def add(a, b):\n    return a + b',
        javascript: 'function add(a, b) {\n    return a + b;\n}',
        java: 'static int add(int a, int b) {\n    return a + b;\n}',
        csharp: 'static int Add(int a, int b) {\n    return a + b;\n}'
      },
      array: {
        cpp: 'std::vector<int> values = {1, 2, 3};',
        rust: 'let values = vec![1, 2, 3];',
        python: 'values = [1, 2, 3]',
        javascript: 'const values = [1, 2, 3];',
        java: 'int[] values = {1, 2, 3};',
        csharp: 'int[] values = {1, 2, 3};'
      },
      string: {
        cpp: 'std::string name = "Alex";',
        rust: 'let name = String::from("Alex");',
        python: 'name = "Alex"',
        javascript: 'const name = "Alex";',
        java: 'String name = "Alex";',
        csharp: 'string name = "Alex";'
      },
      comment: {
        cpp: '// This is a comment',
        rust: '// This is a comment',
        python: '# This is a comment',
        javascript: '// This is a comment',
        java: '// This is a comment',
        csharp: '// This is a comment'
      },
      return: {
        cpp: 'int doubleValue(int n) {\n    return n * 2;\n}',
        rust: 'fn double_value(n: i32) -> i32 {\n    n * 2\n}',
        python: 'def double_value(n):\n    return n * 2',
        javascript: 'function doubleValue(n) {\n    return n * 2;\n}',
        java: 'static int doubleValue(int n) {\n    return n * 2;\n}',
        csharp: 'static int DoubleValue(int n) {\n    return n * 2;\n}'
      }
    };

    const titles = {
      first: "Your first program",
      print: "Printing output",
      variable: "Declaring variables",
      input: "Getting user input",
      condition: "Using if and else",
      loop: "Creating loops",
      function: "Creating functions",
      array: "Working with arrays",
      string: "Working with strings",
      debug: "Debugging your code",
      comment: "Writing comments",
      return: "Returning a value"
    };
    const descriptions = {
      first: "A first program usually displays a short message. This example prints Hello, world!.",
      print: "Use the language's output function or statement to display text.",
      variable: "Variables store values that your program can use later.",
      input: "Input lets a program receive information from the person running it.",
      condition: "An if/else statement chooses which block of code runs based on a condition.",
      loop: "A loop repeats a block of code. This example counts from zero to four.",
      function: "Functions group reusable instructions. This example adds two numbers.",
      array: "Arrays or lists store multiple values in an ordered collection.",
      string: "Strings represent text. This example creates a variable containing a name.",
      debug: "Debugging means finding and correcting the cause of a problem.",
      comment: "Comments explain code and are ignored when the program runs.",
      return: "A return statement sends a value back from a function."
    };
    const key = topic[0];
    const code = examples[key]?.[lang];
    return {
      source: "RUST++ OFFLINE COMMON QUESTIONS",
      title: titles[key],
      message: descriptions[key],
      code: code || undefined,
      explanation: key === "debug"
        ? ["Read the full error message and note the file and line.", "Inspect values around the failing statement.", "Change one thing at a time and test again."]
        : ["This is a small illustrative example.", "Some examples need to be placed inside a complete program or include the appropriate imports."]
    };
  }

  if (command === "analyze_request") {
    const q = String(args.question || "").trim().toLowerCase();

    const rules = [
      [/undefined reference|unresolved external|linker error/, "Undefined reference / linker error", "The linker cannot find the definition of a function or variable.", ["Check that the function or variable is defined.", "Include the source file containing its definition.", "Check library and linker configuration."]],
      [/cannot find symbol|cannot resolve symbol/, "Cannot find symbol", "The compiler cannot resolve a referenced name.", ["Check spelling and capitalization.", "Check scope and declarations.", "Check imports."]],
      [/syntaxerror|syntax error/, "Syntax error", "The parser found code that does not follow the language grammar.", ["Inspect the reported line and the preceding line.", "Check brackets, quotes, and separators.", "Verify the syntax for your selected language."]],
      [/indentationerror|unexpected indent/, "Indentation error", "Python uses indentation to define code blocks.", ["Use consistent indentation.", "Align statements in the same block.", "Avoid mixing tabs and spaces."]],
      [/module not found|no module named|cannot find module/, "Module not found", "The requested module or package could not be located.", ["Check the module name.", "Verify the package is installed.", "Check your import path."]],
      [/borrow|moved value|cannot borrow/, "Rust ownership or borrowing issue", "Rust's ownership rules prevent certain unsafe memory operations.", ["Check whether a value was moved earlier.", "Consider borrowing with & or &mut.", "Check for conflicting mutable borrows."]],
      [/nullreferenceexception|null reference|nil value/, "Null or nil reference", "Your code may be trying to access an absent value.", ["Check whether the value was initialized.", "Add an appropriate null check.", "Inspect where the value is assigned."]],
      [/typeerror/, "Type error", "An operation received a value of an incompatible type.", ["Inspect the types involved.", "Check function arguments and return values.", "Convert values explicitly when appropriate."]],
      [/nameerror|is not defined|not defined/, "Undefined name", "The program references a name unavailable in the current scope.", ["Check spelling and capitalization.", "Make sure the name is declared.", "Check its scope."]],
      [/indexerror|out of range|index out of bounds/, "Index out of range", "The code may be accessing an element outside a collection's valid range.", ["Check the collection length.", "Remember that many languages use zero-based indexing.", "Check loop boundaries."]],
      [/segmentation fault|segfault/, "Segmentation fault", "The program attempted to access memory it was not allowed to access.", ["Inspect pointer and reference usage.", "Check array and buffer boundaries.", "Look for invalid or freed memory access."]]
    ];

    const match = rules.find(([pattern]) => pattern.test(q));

    if (match) {
      return {
        source: "RUST++ CURATED KNOWLEDGE",
        title: match[1],
        message: match[2],
        explanation: match[3]
      };
    }

    return {
      source: "RUST++ LOCAL RULES",
      title: "No matching built-in rule",
      message: q
        ? "The local knowledge base does not contain a specific explanation for this request yet."
        : "Enter a programming question or error message first.",
      explanation: [
        "Try including the exact compiler or runtime error message.",
        "The browser assistant uses curated rules and does not generate answers with AI."
      ]
    };
  }

  throw new Error(`Command not available in browser preview: ${command}`);
}

async function run(command, args, target) {
  try {
    const result = isTauri
      ? await invoke(command, args)
      : browserCommand(command, args);
    showResult(target, result);
  } catch (e) {
    $(target).innerHTML = `<div class="error">${escapeHtml(String(e))}</div>`;
  }
}


editor.onDidChangeModelContent(updateLines);
$("#language").addEventListener("change",()=>{$("#filename").textContent=`untitled.${extension()}`;monaco.editor.setModelLanguage(editor.getModel(),languageIds[$("#language").value]);});
$("#template").onclick=async()=>{const lang=$("#language").value;try{const r=isTauri?await invoke("generate_template",{language:lang}):browserCommand("generate_template",{language:lang});if(r.code)setSource(r.code);updateLines();showResult("#result",r);}catch(e){showResult("#result",{title:"Template unavailable",message:String(e),source:"Rust++"});}};
$("#explain").onclick=()=>run("explain_language",{language:$("#language").value},"#result");
$("#check").onclick=()=>run("validate_source",{language:$("#language").value,source:getSource()},"#result");
$("#run").onclick=async()=>{
  if(!isTauri){
    showResult("#result",{
      source:"RUST++ EXECUTION",
      title:"Native app required",
      message:"Run is available in the installed Rust++ application, not the browser preview.",
      explanation:["Launch Rust++ as a Tauri app to execute code using locally installed compilers and interpreters."]
    });
    return;
  }

  const button=$("#run");
  button.disabled=true;
  button.textContent="Running...";

  try{
    const result=await invoke("run_source",{
      language:$("#language").value,
      source:getSource()
    });
    showResult("#result",result);
  }catch(e){
    showResult("#result",{
      source:"RUST++ EXECUTION",
      title:"Execution failed",
      message:String(e),
      explanation:["Check that the required compiler or interpreter is installed."]
    });
  }finally{
    button.disabled=false;
    button.textContent="▶ Run";
  }
};


const commonQuestions = [
  "How do I write my first program?",
  "How do I print text?",
  "How do I declare variables?",
  "How do I get user input?",
  "How do I use if/else?",
  "How do I create a loop?",
  "How do I create a function?",
  "How do I use arrays?",
  "How do I work with strings?",
  "How do I debug my code?",
  "How do I write comments?",
  "How do I return a value?"
];

$("#common-question-list").innerHTML = commonQuestions.map((q, i) =>
  `<button class="question-chip" data-question="${i}">${escapeHtml(q)}</button>`
).join("");

async function answerCommonQuestion(question) {
  $("#question").value = question;
  await run("answer_common_question", {
    question,
    language: $("#language").value
  }, "#assistant-result");
}

document.querySelectorAll(".question-chip").forEach(button => {
  button.addEventListener("click", () => {
    answerCommonQuestion(commonQuestions[Number(button.dataset.question)]);
  });
});

$("#ask").onclick = () => {
  const question = $("#question").value.trim();
  if (!question) {
    showResult("#assistant-result", {
      title: "Enter a question",
      message: "Type a question or choose one of the common questions above.",
      source: "RUST++"
    });
    return;
  }
  run("analyze_request", { question }, "#assistant-result");
};

$("#save").onclick=async()=>{if(!isTauri){alert("Save is available in the installed Rust++ app. The browser preview cannot access local files.");return;}const path=currentPath||prompt("Save file as (full path):",`untitled.${extension()}`);if(!path)return;try{const msg=await invoke("save_source_file",{path,source:getSource()});currentPath=path;$("#filename").textContent=path.split(/[\\/]/).pop();$("#result").innerHTML=`<div class="success">${escapeHtml(msg)}</div>`;}catch(e){alert(String(e));}};
$("#open").onclick=async()=>{if(!isTauri){alert("Open is available in the installed Rust++ app. The browser preview cannot access local files.");return;}const path=prompt("Enter the full path of the source file to open:");if(!path)return;try{setSource(await invoke("read_source_file",{path}));currentPath=path;$("#filename").textContent=path.split(/[\\/]/).pop();updateLines();}catch(e){alert(String(e));}};
function switchTab(tab){document.querySelectorAll(".view").forEach(x=>x.classList.add("hidden"));$(`#${tab}-view`).classList.remove("hidden");document.querySelectorAll(".nav").forEach(x=>x.classList.toggle("active",x.dataset.tab===tab));$("#page-title").textContent=({editor:"Code editor",assistant:"Assistant",about:"About Rust++"})[tab];}
document.querySelectorAll(".nav").forEach(b=>b.onclick=()=>switchTab(b.dataset.tab));$("#menu").onclick=()=>document.querySelector(".sidebar").classList.toggle("open");
if(isTauri){$("#runtime").textContent="Tauri native app";$("#platform-label").textContent="Desktop / mobile workspace";invoke("app_info").then(info=>console.info("Rust++",info));}
updateLines();
