# Tokenizer

Tokenizer is a Rust project inspired by OpenAI, aimed at providing a basic implementation of the Byte Pair Encoding (BPE) algorithm. This project serves as a learning opportunity for Rust enthusiasts, particularly those interested in the field of artificial intelligence.

## Introduction

Byte Pair Encoding (BPE) is a popular technique used in natural language processing (NLP) tasks, particularly in tokenization. It involves iteratively merging the most frequent pair of symbols in a corpus, effectively learning subword units that are useful for various NLP tasks.

Tokenizer implements the BPE algorithm in Rust, providing a foundation for further exploration and experimentation in tokenization and NLP.

## Features

-   **BPE Implementation**: Provides a basic implementation of the Byte Pair Encoding algorithm in Rust.
-   **Extensible**: Designed with modularity in mind, allowing for easy expansion with additional modules for different tokenization techniques, such as GPT-2 tokenization.
-   **MIT License**: Released under the MIT License, enabling anyone to use, modify, and distribute the project freely.

## Usage

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/usama3627/tokenizer.git
    ```

2. **Install Rust**:
   Ensure that you have Rust installed on your system. You can install it using [rustup](https://rustup.rs/).

3. **Build and Run**:

    ```bash
    cd tokenizer
    cargo build
    cargo run
    ```

4. **Training Data**:
   To train the tokenizer, download the training dataset from the provided [Link (huggingface dataset)](https://datasets-server.huggingface.co/rows?dataset=tweet_eval&config=emoji&split=train&offset=0&length=100) and rename it to `myresponse.json`. Place the dataset in the project directory. For testing, I am using 100 rows of tweets.

## Dependencies

-   serde = { version = "1.0.104", features = ["derive"] }
-   serde_json = "1.0.48"

Ensure these dependencies are specified in your `Cargo.toml` file.

## Future Work

In future iterations of the project, the following enhancements can be considered:

-   ~~**Modularization**: Refactor the code into modules, separating concerns such as BPE implementation and GPT-2 tokenization.~~
-   **Optimizations**: Explore optimizations to improve the performance of the tokenization process.
-   **Documentation**: Enhance documentation to provide detailed explanations of the algorithms and codebase.
-   **Additional Tokenization Techniques**: Integrate additional tokenization techniques to provide a comprehensive toolkit for NLP tasks.

## License

Tokenizer is released under the [MIT License](LICENSE). Feel free to use, modify, and distribute the project according to the terms of the license.

## Contributors

-   [Usama Mehmood](https://github.com/usama3627)

Contributions to the project are welcome. Fork the repository, make your changes, and submit a pull request.

## Acknowledgements

Tokenizer was inspired by the work of OpenAI and Andrej Karpathy and aims to contribute to the Rust and AI communities.
