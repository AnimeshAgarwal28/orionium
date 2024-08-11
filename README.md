# Orionium
### A rust 🦀 based browser engine.

Orionium is a browser engine written in Rust. For now the project aims to provide a foundation for parsing HTML, constructing a DOM, and rendering web pages. As it evolves, Orionium will serve as the backbone for a fully-featured web browser, supporting HTML, CSS, and potentially JavaScript execution.

## Implemented Features

- **HTML Parsing**: Convert raw HTML strings into a structured DOM tree.
- **DOM Tree Construction**: Build a DOM tree from parsed HTML tokens.
- **Rendering**: Render the DOM tree, preparing it for display.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (ensure you have the latest version installed)

### Building and Running

1. **Clone the repository:**

   ```bash
   git clone https://github.com/AnimeshAgarwal28/orionium.git
   cd orionium
   ```

2. **Build the project:**

   ```bash
   cargo build
   ```

3. **Run the project:**

   ```bash
   cargo run
   ```

   The program will parse a hardcoded HTML string, build a DOM tree, and render it back to HTML.

## Scope of the Project

Orionium is currently focused on the core components of a browser engine:

- **HTML Parsing**: Building a robust HTML parser.
- **DOM Manipulation**: Creating a complete and manipulable DOM tree.
- **Rendering**: Preparing the DOM tree for display.

### Future Goals

- **CSS Parsing and Rendering**: Adding support for CSS to style the DOM.
- **JavaScript Execution**: Integrating a JavaScript engine to handle dynamic content.
- **Web Browser Integration**: Expanding Orionium into a full-fledged web browser engine.

## Contributing

We welcome contributions from the community! Here are a few guidelines to help you get started:

1. **Fork the repository** and create a new branch for your feature or bugfix.
2. **Implement tests** for any new functionality you add.
3. **Document your changes** in the code and update the `README.md` if necessary.
4. **Submit a pull request** with a clear description of your changes.

### Development Guidelines

- **Code Style**: Please follow Rust's standard formatting (`cargo fmt`) and linting (`cargo clippy`) guidelines.
- **Commit Messages**: Write clear and concise commit messages.
- **Issue Tracker**: Use GitHub Issues to report bugs or suggest features.

### Writing Tests

As the project evolves, tests should be added to ensure that new and existing functionality works correctly. Please use Rust's built-in testing framework for writing and running tests:

- **Unit Tests**: Place unit tests within the same file as the code they are testing, in a `#[cfg(test)]` module.
- **Integration Tests**: Use the `tests` directory for integration tests that check how different parts of the engine work together.

We encourage contributors to add tests when implementing new features or fixing bugs.

### Development Guidelines

- **Code Style**: Please follow Rust's standard formatting (`cargo fmt`) and linting (`cargo clippy`) guidelines.
- **Commit Messages**: Write clear and concise commit messages.
- **Issue Tracker**: Use the issue tracker to report bugs or suggest features.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/AnimeshAgarwal28/orionium/blob/master/LICENSE) file for more details.
