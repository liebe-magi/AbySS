# **AbySS: Advanced-scripting by Symbolic Syntax**

AbySS (Advanced-scripting by Symbolic Syntax) is a programming language designed to combine the thrill of casting spells with the power of advanced scripting. AbySS aims to provide an intuitive and symbolically rich syntax that allows developers to interact with their code as if they were performing magic. Whether you're scripting a simple operation or crafting complex systems, AbySS offers a unique and immersive experience.

## **Key Features**

- **Symbolic Syntax**: AbySS emphasizes a symbolically intuitive syntax, making the code easy to read and write, while retaining powerful functionality.
- **Spellcasting-inspired Programming**: The language's design mimics the experience of casting spells, with reserved keywords that evoke a magical theme.
- **Interactive Spellcasting**: AbySS supports interactive scripting through an interpreter, allowing real-time execution and feedback.
- **Structured Sorcery**: AbySS encourages structured programming, combining the flexibility of scripting with the rigor of structured code.

## **Table of Contents**
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Language Syntax](#language-syntax)
  - [Basic Syntax](#basic-syntax)
  - [Types](#types)
  - [Variable Declaration](#variable-declaration)
  - [Conditionals](#conditionals)
  - [Functions](#functions)
  - [Input/Output](#inputoutput)
- [Examples](#examples)
- [Roadmap](#roadmap)
- [License](#license)

## **Installation**
(TBD)
AbySS installation instructions will be provided as the project matures. For now, you can clone the repository and build it locally.

```bash
git clone https://github.com/your-repository/abyss.git
cd abyss
cargo build
```

## **Getting Started**
To start using AbySS, you can either enter the interactive interpreter mode or run `.aby` script files. (Work-in-progress)

### **Running the Interpreter**
You can start the AbySS interpreter (currently under development) by using the following command:

```bash
abyss cast
```

### **Running Scripts**
To run a `.aby` script file, use the following command:

```bash
abyss invoke <script.aby>
```

## **Language Syntax**

### **Basic Syntax**
AbySS uses a symbolic and intuitive syntax inspired by magical themes. Below are some of the core elements of the language.

- **Comments**: Comments in AbySS are marked with `//` for single-line comments and `/* */` for multi-line comments.

```abyss
// This is a single-line comment
/*
  This is a multi-line comment
*/
```

### **Types**
AbySS supports the following primitive types:
- **arcana**: Represents integers (e.g., `42`, `-3`).
- **aether**: Represents floating-point numbers (TBD).
- **rune**: Represents strings (e.g., `"Hello, World"`).
- **omen**: Represents boolean values, with `grace` for `true` and `curse` for `false`.
- **abyss**: Represents the `void` type, indicating no value.

```abyss
forge x: arcana = 10;
forge message: rune = "Hello, AbySS";
forge is_active: omen = grace;
```

### **Variable Declaration**
Variables are declared using the `forge` keyword. You must explicitly specify the variable type.

```abyss
forge x: arcana = 42;
forge greeting: rune = "Welcome to AbySS!";
```

### **Conditionals**
AbySS does not use traditional `if` statements. Instead, it uses the `oracle` construct, inspired by match statements in other languages. (Work-in-progress)

```abyss
oracle x {
  1 => proclaim("One"),
  2 => proclaim("Two"),
  _ => proclaim("Something else")
}
```

### **Functions**
Functions in AbySS are defined using the `engrave` keyword. (Work-in-progress)

```abyss
engrave add(a: arcana, b: arcana): arcana {
  a + b
}
```

### **Input/Output**
For output, AbySS uses the `unveil` function to print values to the console.

```abyss
unveil("Hello, AbySS");
unveil(x + 42);
```

Input functionality is TBD.

## **Examples**

### **Basic Arithmetic**
```abyss
forge x: arcana = 10;
forge y: arcana = 20;
unveil(x + y);
```

### **Conditionals with `oracle`**
```abyss
forge num: arcana = 3;

oracle num {
  1 => unveil("One"),
  2 => unveil("Two"),
  _ => unveil("Other number")
}
```

### **Functions**
```abyss
engrave greet(name: rune): abyss {
  unveil("Hello, " + name);
}

greet("AbySS");
```

## **Roadmap**
- **Interpreter Improvements**: Enhance the interpreter for better real-time feedback (Work-in-progress).
- **Type System Expansion**: Implement additional types like `aether` (Work-in-progress).
- **File I/O**: Introduce input functionality and file handling (TBD).
- **Standard Library**: Develop a standard library with common functions and utilities (TBD).
- **Error Handling**: Implement robust error handling (TBD).

## **License**
AbySS is open-source software licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

This README provides an overview of AbySS, its syntax, and usage. Please note that many features are still under development, and the language is actively evolving. Contributions and feedback are welcome!