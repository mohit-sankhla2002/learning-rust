# Rust Package Structure

In Rust, a package is a fundamental organisational unit for code. It consists of several components that work together to build, test, and share Rust code. The essential components of a Rust package include:

### Packages

Packages contain one or more crates and provide a set of functionality. They are the top-level containers for organizing Rust code. Packages are used to manage and distribute code.

### Cargo.toml

The `Cargo.toml` file is a crucial part of a Rust package. It describes the package and defines how to build crates within the package. It contains metadata about the package, its dependencies, and other configuration options.

### Rules

Packages in Rust must adhere to certain rules:

* **Must Have at Least 1 Crate**: A package should contain at least one crate. A crate is a unit of compilation in Rust.
* **At Most 1 Library Crate**: A package can have at most one library crate. The library crate is used to define reusable code that can be used in other crates or projects.
* **Any Number of Binary Crates**: A package can contain any number of binary crates. Binary crates are used to produce executable programs.

### Crates

Crates are the building blocks of a Rust package. They form a tree-like structure of modules, and they are the units of compilation. Crates can produce libraries or executables, depending on their purpose.

### Modules

Modules in Rust allow you to control the organization, scope, and structure of your code. They help you break down your code into smaller, manageable pieces and provide a way to structure your project logically.


## More on Modules
- Organize code for readability and reuse 
- Control Scope and Privacy 
- Contains items(functions, structs, enums, traits, etc)
- Explicitly Defined (using the mod keyword)
    - Not mapped to the **file system**
    - Flexibility & straight forward conditional compilation
