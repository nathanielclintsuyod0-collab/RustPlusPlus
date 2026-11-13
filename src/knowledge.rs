use crate::{language::Language, AssistantReply};

pub fn template(lang: Language) -> AssistantReply {
    let (name, code, explanation) = match lang {
        Language::Cpp => ("C++ starter", r#"#include <iostream>

int main() {
    std::cout << "Hello, world!\n";
    return 0;
}"#, vec![
            "`#include <iostream>` makes standard input/output facilities available.".into(),
            "`main` is the program entry point.".into(),
            "`std::cout` writes text to standard output.".into(),
            "`return 0` signals successful completion.".into(),
        ]),
        Language::Rust => ("Rust starter", r#"fn main() {
    println!("Hello, world!");
}"#, vec![
            "`fn main()` defines the executable entry point.".into(),
            "`println!` is a standard-library macro that prints a line.".into(),
        ]),
        Language::Python => ("Python starter", r#"def main():
    print("Hello, world!")

if __name__ == "__main__":
    main()"#, vec![
            "`def main()` groups the program's starting logic in a function.".into(),
            "`print` writes text to the console.".into(),
            "The `__name__` guard runs `main` when this file is executed directly.".into(),
        ]),
        Language::JavaScript => ("JavaScript starter", r#"function main() {
  console.log("Hello, world!");
}

main();"#, vec![
            "`function main()` declares a function.".into(),
            "`console.log` writes a message to the console.".into(),
            "`main()` invokes the function.".into(),
        ]),
        Language::Java => ("Java starter", r#"public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}"#, vec![
            "Java source declares a class; this example uses `Main`.".into(),
            "`main` is the conventional application entry point.".into(),
            "`System.out.println` prints a line.".into(),
        ]),
        Language::CSharp => ("C# starter", r#"using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, world!");
    }
}"#, vec![
            "`using System` imports common .NET types.".into(),
            "`Main` is the application entry point in this example.".into(),
            "`Console.WriteLine` prints a line.".into(),
        ]),
    };
    AssistantReply {
        title: name.into(),
        message: format!("Here is a built-in {} starter template.", lang.display()),
        code: Some(code.into()),
        explanation,
        source: "bundled template library".into(),
    }
}

pub fn explain_language(lang: Language) -> AssistantReply {
    let explanation = match lang {
        Language::Cpp => vec!["C++ is a compiled, general-purpose language.".into(), "Common build tools include Clang and GCC.".into(), "This assistant's built-in C++ support currently covers templates and known-rule suggestions, not the full C++ standard.".into()],
        Language::Rust => vec!["Rust emphasizes memory safety through ownership and borrowing.".into(), "Cargo manages Rust packages, builds, and tests.".into(), "The compiler `rustc` provides authoritative type and borrow checking.".into()],
        Language::Python => vec!["Python is a high-level interpreted language.".into(), "Indentation defines code blocks.".into(), "The standard interpreter is `python` or `python3`.".into()],
        Language::JavaScript => vec!["JavaScript is widely used in browsers and server runtimes such as Node.js.".into(), "Use `const` by default and `let` when reassignment is needed.".into(), "The runtime or browser reports execution errors.".into()],
        Language::Java => vec!["Java is a statically typed, class-based language.".into(), "The JDK includes `javac` and the Java runtime.".into(), "A public class is normally stored in a file with the same name.".into()],
        Language::CSharp => vec!["C# is a statically typed language in the .NET ecosystem.".into(), "The .NET SDK provides `dotnet build` and `dotnet run`.".into(), "Types, namespaces, and project configuration affect compilation.".into()],
    };
    AssistantReply {
        title: format!("{} quick guide", lang.display()),
        message: format!("These are predefined notes about {}.", lang.display()),
        code: None,
        explanation,
        source: "bundled knowledge base".into(),
    }
}

pub fn known_error(question: &str) -> Option<AssistantReply> {
    let q = question.to_ascii_lowercase();
    let (title, message, tips): (&str, &str, Vec<&str>) =
        if q.contains("undefined reference") || q.contains("unresolved external") {
            ("Linker error: undefined reference",
             "The linker cannot find a definition for a symbol your program uses.",
             vec!["Confirm the function or global variable has a matching definition.", "Ensure the source file containing the definition is included in the build.", "Check that declaration and definition signatures match.", "For external libraries, verify the library is installed and linked."])
        } else if q.contains("segmentation fault") || q.contains("access violation") {
            ("Invalid memory access",
             "The program attempted to access memory it was not permitted to access.",
             vec!["Check pointers before dereferencing them.", "Check array indexes and iterator validity.", "Avoid using objects after their lifetime ends.", "Use a debugger or memory sanitizer to locate the failing access."])
        } else if q.contains("cannot find") && (q.contains("module") || q.contains("crate") || q.contains("package")) {
            ("Dependency or module not found",
             "The compiler or runtime cannot resolve a referenced dependency or module.",
             vec!["Check the dependency name and spelling.", "Install or declare the dependency using the language's package manager.", "Verify the module search path and project configuration."])
        } else if q.contains("borrow") && q.contains("rust") {
            ("Rust borrowing suggestion",
             "A borrow-checker diagnostic usually means a reference would outlive its data or conflict with another borrow.",
             vec!["Read the exact compiler span first.", "Avoid keeping a reference while mutating the same value.", "Consider taking ownership, shortening the borrow's scope, or cloning only when appropriate."])
        } else {
            return None;
        };
    Some(AssistantReply {
        title: title.into(),
        message: message.into(),
        code: None,
        explanation: tips.into_iter().map(str::to_string).collect(),
        source: "predefined diagnostic rules".into(),
    })
}

/// Answers a curated common programming question using the selected language.
pub fn answer_common_question(question: &str, lang: Language) -> Option<AssistantReply> {
    let q = question.to_ascii_lowercase();

    let topic = if q.contains("first program") || q.contains("hello world") || q.contains("getting started") {
        "first"
    } else if q.contains("print") || q.contains("output") || q.contains("display text") {
        "print"
    } else if q.contains("variable") {
        "variable"
    } else if q.contains("input") || q.contains("user input") || q.contains("read input") {
        "input"
    } else if q.contains("if") || q.contains("condition") || q.contains("else") {
        "condition"
    } else if q.contains("loop") || q.contains("repeat") || q.contains("iteration") {
        "loop"
    } else if q.contains("function") || q.contains("method") {
        "function"
    } else if q.contains("array") || q.contains("list") || q.contains("collection") {
        "array"
    } else if q.contains("string") || q.contains("text") {
        "string"
    } else if q.contains("debug") || q.contains("fix an error") || q.contains("find an error") {
        "debug"
    } else if q.contains("comment") {
        "comment"
    } else if q.contains("return") {
        "return"
    } else {
        return None;
    };

    let (title, message, code, notes): (&str, &str, &str, Vec<&str>) = match (topic, lang) {
        ("first", Language::Cpp) => ("Your first C++ program", "Compile and run this small program to get started.", "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, world!\\n\";\n    return 0;\n}", vec!["Save the file with a .cpp extension.", "Compile it with a C++ compiler such as g++.", "Run the compiled executable."]),
        ("first", Language::Rust) => ("Your first Rust program", "Rust programs commonly start execution in main.", "fn main() {\n    println!(\"Hello, world!\");\n}", vec!["Save the file with a .rs extension.", "Compile with rustc or use Cargo.", "Run the resulting executable."]),
        ("first", Language::Python) => ("Your first Python program", "Python can run a source file directly.", "print(\"Hello, world!\")", vec!["Save the file with a .py extension.", "Run it with python or python3."]),
        ("first", Language::JavaScript) => ("Your first JavaScript program", "This example runs in a browser console or Node.js.", "console.log(\"Hello, world!\");", vec!["Save the file with a .js extension.", "Run it with Node.js or in a browser."]),
        ("first", Language::Java) => ("Your first Java program", "Java applications commonly begin in a class containing main.", "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, world!\");\n    }\n}", vec!["Save this as Main.java.", "Compile with javac Main.java.", "Run with java Main."]),
        ("first", Language::CSharp) => ("Your first C# program", "This example uses the traditional Main entry point.", "using System;\n\nclass Program {\n    static void Main() {\n        Console.WriteLine(\"Hello, world!\");\n    }\n}", vec!["Place the code in a .NET project.", "Build with dotnet build.", "Run with dotnet run."]),

        ("print", Language::Cpp) => ("Printing text in C++", "Use std::cout to write text to standard output.", "#include <iostream>\nstd::cout << \"Hello!\" << std::endl;", vec!["Include iostream.", "Use quotation marks around string literals."]),
        ("print", Language::Rust) => ("Printing text in Rust", "Use println! to print a line.", "println!(\"Hello!\");", vec!["println! adds a newline.", "Use print! when you do not want an automatic newline."]),
        ("print", Language::Python) => ("Printing text in Python", "Use the built-in print function.", "print(\"Hello!\")", vec!["Separate multiple values with commas.", "print adds a newline by default."]),
        ("print", Language::JavaScript) => ("Printing text in JavaScript", "Use console.log for console output.", "console.log(\"Hello!\");", vec!["This is useful for debugging and simple output."]),
        ("print", Language::Java) => ("Printing text in Java", "Use System.out.println to print a line.", "System.out.println(\"Hello!\");", vec!["The statement ends with a semicolon."]),
        ("print", Language::CSharp) => ("Printing text in C#", "Use Console.WriteLine to print a line.", "Console.WriteLine(\"Hello!\");", vec!["Include using System; or qualify the Console type."]),

        ("variable", Language::Cpp) => ("Declaring C++ variables", "Specify a type before the variable name.", "int age = 18;\nstd::string name = \"Alex\";", vec!["Use a type that matches the value.", "Include string support when using std::string."]),
        ("variable", Language::Rust) => ("Declaring Rust variables", "Use let to declare a variable.", "let age: i32 = 18;\nlet name = \"Alex\";", vec!["Variables are immutable by default.", "Use let mut when reassignment is needed."]),
        ("variable", Language::Python) => ("Declaring Python variables", "Assign a value to a name; Python infers its type at runtime.", "age = 18\nname = \"Alex\"", vec!["You do not need to declare a type for ordinary assignments."]),
        ("variable", Language::JavaScript) => ("Declaring JavaScript variables", "Use const by default and let when reassignment is required.", "const name = \"Alex\";\nlet age = 18;", vec!["Avoid var in modern JavaScript unless maintaining older code."]),
        ("variable", Language::Java) => ("Declaring Java variables", "Declare a type followed by a variable name.", "int age = 18;\nString name = \"Alex\";", vec!["Java is statically typed.", "The declared type constrains the values assigned."]),
        ("variable", Language::CSharp) => ("Declaring C# variables", "Declare a type or use var for local type inference.", "int age = 18;\nstring name = \"Alex\";", vec!["C# is statically typed.", "Use var when the initializer makes the type clear."]),

        ("input", Language::Cpp) => ("Getting input in C++", "Use std::cin to read a value from standard input.", "#include <iostream>\nint age;\nstd::cin >> age;", vec!["Declare the variable before reading into it.", "Use getline for a full line of text."]),
        ("input", Language::Rust) => ("Getting input in Rust", "Read a line into a mutable String.", "use std::io;\nlet mut input = String::new();\nio::stdin().read_line(&mut input).expect(\"Input failed\");", vec!["The input string includes its newline when present.", "Parse the string to convert it into a number."]),
        ("input", Language::Python) => ("Getting input in Python", "Use input to read a line of text.", "name = input(\"Enter your name: \")", vec!["input returns a string.", "Convert numeric input with int or float."]),
        ("input", Language::JavaScript) => ("Getting input in JavaScript", "In Node.js, readline can collect terminal input.", "const readline = require(\"node:readline\");\nconst rl = readline.createInterface({ input: process.stdin, output: process.stdout });\nrl.question(\"Name: \", answer => {\n  console.log(answer);\n  rl.close();\n});", vec!["Browser input is commonly collected with form controls or prompt.", "Node.js terminal input uses a separate API."]),
        ("input", Language::Java) => ("Getting input in Java", "Scanner can read values from standard input.", "import java.util.Scanner;\nScanner scanner = new Scanner(System.in);\nString name = scanner.nextLine();", vec!["Import java.util.Scanner.", "Close the scanner when it is no longer needed."]),
        ("input", Language::CSharp) => ("Getting input in C#", "Console.ReadLine reads a line from standard input.", "string? name = Console.ReadLine();", vec!["ReadLine returns text or null.", "Parse the text when a number is required."]),

        ("condition", Language::Cpp) => ("C++ if and else", "Use if and else to choose which block runs.", "if (age >= 18) {\n    std::cout << \"Adult\";\n} else {\n    std::cout << \"Minor\";\n}", vec!["The condition is evaluated as true or false."]),
        ("condition", Language::Rust) => ("Rust if and else", "Rust uses if expressions to choose a branch.", "if age >= 18 {\n    println!(\"Adult\");\n} else {\n    println!(\"Minor\");\n}", vec!["Each branch body is enclosed in braces."]),
        ("condition", Language::Python) => ("Python if and else", "Python uses indentation to define conditional blocks.", "if age >= 18:\n    print(\"Adult\")\nelse:\n    print(\"Minor\")", vec!["End the condition line with a colon.", "Keep indentation consistent."]),
        ("condition", Language::JavaScript) => ("JavaScript if and else", "Use if and else with a boolean condition.", "if (age >= 18) {\n  console.log(\"Adult\");\n} else {\n  console.log(\"Minor\");\n}", vec!["Use === for strict equality comparisons."]),
        ("condition", Language::Java) => ("Java if and else", "Use if and else to select a code path.", "if (age >= 18) {\n    System.out.println(\"Adult\");\n} else {\n    System.out.println(\"Minor\");\n}", vec!["The condition must evaluate to a boolean."]),
        ("condition", Language::CSharp) => ("C# if and else", "Use if and else to select a code path.", "if (age >= 18) {\n    Console.WriteLine(\"Adult\");\n} else {\n    Console.WriteLine(\"Minor\");\n}", vec!["The condition must evaluate to a bool."]),

        ("loop", Language::Cpp) => ("C++ loops", "A for loop repeats while its condition remains true.", "for (int i = 0; i < 5; i++) {\n    std::cout << i << std::endl;\n}", vec!["The example prints values from 0 through 4."]),
        ("loop", Language::Rust) => ("Rust loops", "A for loop can iterate over a range.", "for i in 0..5 {\n    println!(\"{}\", i);\n}", vec!["The range 0..5 excludes 5."]),
        ("loop", Language::Python) => ("Python loops", "Use for to iterate over a range of numbers.", "for i in range(5):\n    print(i)", vec!["range(5) produces 0 through 4."]),
        ("loop", Language::JavaScript) => ("JavaScript loops", "A for loop repeats while its condition remains true.", "for (let i = 0; i < 5; i++) {\n  console.log(i);\n}", vec!["The update expression increments i after each iteration."]),
        ("loop", Language::Java) => ("Java loops", "A for loop repeats while its condition remains true.", "for (int i = 0; i < 5; i++) {\n    System.out.println(i);\n}", vec!["The loop variable is commonly declared in the initializer."]),
        ("loop", Language::CSharp) => ("C# loops", "A for loop repeats while its condition remains true.", "for (int i = 0; i < 5; i++) {\n    Console.WriteLine(i);\n}", vec!["The loop variable is commonly declared in the initializer."]),

        ("function", Language::Cpp) => ("C++ functions", "Declare a function with a return type and name.", "int add(int a, int b) {\n    return a + b;\n}", vec!["Call it with add(2, 3)."]),
        ("function", Language::Rust) => ("Rust functions", "Use fn to declare a function.", "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}", vec!["The arrow specifies the return type.", "The final expression is returned without a semicolon."]),
        ("function", Language::Python) => ("Python functions", "Use def to define a function.", "def add(a, b):\n    return a + b", vec!["Call it with add(2, 3)."]),
        ("function", Language::JavaScript) => ("JavaScript functions", "Use the function keyword to define a function.", "function add(a, b) {\n  return a + b;\n}", vec!["Call it with add(2, 3)."]),
        ("function", Language::Java) => ("Java methods", "Methods are declared inside a class.", "static int add(int a, int b) {\n    return a + b;\n}", vec!["This example is a static method that can be called from the class."]),
        ("function", Language::CSharp) => ("C# methods", "Methods are declared inside a class or type.", "static int Add(int a, int b) {\n    return a + b;\n}", vec!["This example is a static method."]),

        ("array", Language::Cpp) => ("C++ collections", "std::vector is a resizable sequence.", "#include <vector>\nstd::vector<int> values = {1, 2, 3};", vec!["Include the vector header.", "Use values[0] to access the first element."]),
        ("array", Language::Rust) => ("Rust collections", "An array has a fixed length; Vec can grow.", "let values = vec![1, 2, 3];", vec!["Use values[0] to access the first element.", "Use Vec when the collection needs to grow."]),
        ("array", Language::Python) => ("Python collections", "A list stores an ordered collection of values.", "values = [1, 2, 3]", vec!["Use values[0] for the first item.", "Lists can grow and shrink."]),
        ("array", Language::JavaScript) => ("JavaScript arrays", "An array stores an ordered collection of values.", "const values = [1, 2, 3];", vec!["Use values[0] for the first item.", "Use push to append an item."]),
        ("array", Language::Java) => ("Java arrays", "An array stores a fixed-size sequence of one type.", "int[] values = {1, 2, 3};", vec!["Use values[0] for the first item.", "Use values.length to get its size."]),
        ("array", Language::CSharp) => ("C# arrays", "An array stores a fixed-size sequence of one type.", "int[] values = { 1, 2, 3 };", vec!["Use values[0] for the first item.", "Use values.Length to get its size."]),

        ("string", Language::Cpp) => ("C++ strings", "Use std::string for ordinary text.", "#include <string>\nstd::string name = \"Alex\";", vec!["Include the string header."]),
        ("string", Language::Rust) => ("Rust strings", "String stores owned, growable UTF-8 text.", "let name = String::from(\"Alex\");", vec!["String::from creates an owned String.", "String slices use the &str type."]),
        ("string", Language::Python) => ("Python strings", "Text values are represented by str.", "name = \"Alex\"\ngreeting = \"Hello, \" + name", vec!["Strings are immutable.", "Use f-strings for formatted text."]),
        ("string", Language::JavaScript) => ("JavaScript strings", "String values can use single, double, or template quotes.", "const name = \"Alex\";\nconst greeting = `Hello, ${name}`;", vec!["Template literals use backticks and ${...} interpolation."]),
        ("string", Language::Java) => ("Java strings", "Use String for text.", "String name = \"Alex\";\nString greeting = \"Hello, \" + name;", vec!["String values are immutable."]),
        ("string", Language::CSharp) => ("C# strings", "Use string for text.", "string name = \"Alex\";\nstring greeting = $\"Hello, {name}\";", vec!["Interpolated strings use a $ prefix."]),

        ("debug", _) => ("Debugging your code", "Debugging is the process of locating and correcting the cause of a problem.", "", vec!["Read the complete error message and identify the reported file and line.", "Reproduce the problem with the smallest input possible.", "Inspect variable values around the failing statement.", "Change one thing at a time and test again."]),
        ("comment", Language::Python) => ("Python comments", "Use # to write a single-line comment.", "# This is a comment", vec!["Comments are ignored by the Python interpreter."]),
        ("comment", Language::Rust) => ("Rust comments", "Use // for a single-line comment.", "// This is a comment", vec!["Rust also supports block comments with /* ... */."]),
        ("comment", Language::Cpp | Language::JavaScript | Language::Java | Language::CSharp) => ("Code comments", "Use // for a single-line comment.", "// This is a comment", vec!["Block comments can use /* ... */."]),
        ("return", Language::Cpp) => ("Returning a value in C++", "Use return followed by a value matching the function's return type.", "int doubleValue(int number) {\n    return number * 2;\n}", vec!["The function signature declares int as its return type."]),
        ("return", Language::Rust) => ("Returning a value in Rust", "Return a value with return or use the final expression.", "fn double_value(number: i32) -> i32 {\n    number * 2\n}", vec!["The final expression is returned because it has no semicolon."]),
        ("return", Language::Python) => ("Returning a value in Python", "Use return inside a function.", "def double_value(number):\n    return number * 2", vec!["A function without return produces None."]),
        ("return", Language::JavaScript) => ("Returning a value in JavaScript", "Use return inside a function.", "function doubleValue(number) {\n  return number * 2;\n}", vec!["Code after return in the same function is not executed."]),
        ("return", Language::Java) => ("Returning a value in Java", "Declare the return type and return a matching value.", "static int doubleValue(int number) {\n    return number * 2;\n}", vec!["A void method does not return a value."]),
        ("return", Language::CSharp) => ("Returning a value in C#", "Declare the return type and return a matching value.", "static int DoubleValue(int number) {\n    return number * 2;\n}", vec!["A void method does not return a value."]),

        _ => return None,
    };

    Some(AssistantReply {
        title: title.into(),
        message: message.into(),
        code: if code.is_empty() { None } else { Some(code.into()) },
        explanation: notes.into_iter().map(str::to_string).collect(),
        source: "offline common questions library".into(),
    })
}
