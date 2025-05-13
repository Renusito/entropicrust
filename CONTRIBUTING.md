# Contributing to EntropicRust

First off, thank you for considering contributing to EntropicRust! This project thrives on community involvement, and your help is greatly appreciated. Whether it's reporting a bug, suggesting a new chaotic system, improving visualizations, or fixing code, every contribution matters.

Please take a moment to review these guidelines.

## Code of Conduct

To ensure a welcoming and inclusive environment for everyone, EntropicRust adheres to the following Code of Conduct. By participating in this project, you agree to abide by its terms.

## How Can I Contribute?

There are many ways to help improve EntropicRust:

*   **Reporting Bugs:** Found something behaving unexpectedly? A visual glitch? A crash when changing parameters? Let us know by opening an issue.
*   **Suggesting Enhancements:** Have ideas for making the simulator better?
    *   Adding more chaotic systems (e.g., Halvorsen, Thomas, Dadras)?
    *   Implementing more accurate numerical solvers (like Runge-Kutta 4)?
    *   Improving visualization (camera controls, color schemes based on velocity, different 2D projections)?
    *   Enhancing the UI or controls?
    Open an issue on GitHub to discuss your ideas!
*   **Writing Documentation:** Improve the README.md, add code comments to explain complex sections, or clarify usage instructions.
*   **Submitting Code:** Fix known bugs or implement approved features/enhancements via Pull Requests.

## Getting Started

Ready to contribute code? Here’s how to set up:

1.  **Fork the repository** on GitHub: Go to `https://github.com/emanuellcs/entropicrust` and click the "Fork" button.
2.  **Clone your fork** locally:
    ```bash
    git clone https://github.com/YOUR_USERNAME/entropicrust.git
    cd entropicrust
    ```
    *(Replace `YOUR_USERNAME` with your actual GitHub username)*
3.  **Add the original repository as an upstream remote:** This allows you to pull changes from the main project later.
    ```bash
    git remote add upstream https://github.com/emanuellcs/entropicrust.git
    ```
4.  **Ensure you can build and run the project.** Refer to the `README.md` for any specific dependencies needed by `ggez` on your system.
    ```bash
    cargo build
    cargo run --release # Recommended for performance testing
    ```
5.  **Create a new branch** for your specific contribution. Use a descriptive name:
    ```bash
    # Example for a new feature:
    # git checkout -b feature/add-runge-kutta
    # Example for a bug fix:
    # git checkout -b fix/aizawa-parameter-scaling
    git checkout -b your-descriptive-branch-name
    ```

## Reporting Bugs

Before submitting a bug report:

1.  **Search existing issues:** Check if the bug has already been reported on the [Issues tab](https://github.com/emanuellcs/entropicrust/issues).
2.  If it's a new bug, please [open a new issue](https://github.com/emanuellcs/entropicrust/issues/new). Include:
    *   A clear, descriptive title (e.g., "Crash when decreasing particle count below 5").
    *   Detailed steps to reproduce the bug.
    *   What you expected to happen.
    *   What actually happened (include error messages or screenshots if possible).
    *   Your operating system and Rust version (`rustc --version`).

## Suggesting Enhancements

1.  **Check existing issues:** See if your idea has already been suggested or discussed on the [Issues tab](https://github.com/emanuellcs/entropicrust/issues).
2.  If it's a new idea, [open a new issue](https://github.com/emanuellcs/entropicrust/issues/new). Describe:
    *   The enhancement you're proposing.
    *   Why it would be valuable (e.g., "Adding RK4 would improve simulation accuracy").
    *   Any initial thoughts on implementation (optional).

## Submitting Pull Requests (PRs)

When your code changes are ready:

1.  **Format and Lint:** Ensure your code adheres to standard Rust style:
    ```bash
    cargo fmt --all
    cargo clippy --all-targets -- -D warnings
    ```
    Address any formatting issues or clippy warnings.
2.  **Test:** Confirm the project still builds (`cargo build`) and runs correctly (`cargo run --release`). Test the specific changes you made.
3.  **Update Documentation:** If your PR changes user-facing behavior, adds features, or modifies controls, update the `README.md` accordingly. Add code comments where necessary.
4.  **Commit Changes:** Write clear, concise commit messages describing your changes.
5.  **Push Branch:** Push your local branch to your fork on GitHub:
    ```bash
    git push origin your-descriptive-branch-name
    ```
6.  **Open a Pull Request:** Go to the `emanuellcs/entropicrust` repository on GitHub. You should see a prompt to open a PR from your recently pushed branch.
    *   Provide a clear title and a detailed description of your changes.
    *   Explain *why* the changes were made.
    *   Link to any relevant issues (e.g., "Closes #15" or "Implements suggestion in #22").
7.  **Code Review:** Be prepared to discuss your code and make adjustments based on feedback from the maintainer(s).

## Coding Style

*   Follow standard Rust conventions and idioms.
*   Use `rustfmt` for automatic formatting.
*   Address warnings reported by `clippy`.
*   Write clear, readable code. Add comments to explain non-obvious logic.

## License

By contributing to EntropicRust, you agree that your contributions will be licensed under the project's existing [MIT License](LICENSE).

---

Thank you for helping make EntropicRust a better tool for exploring chaos!