# Rust To-Do Program

  

A simple to-do program written in Rust using the `clap` crate for command-line argument parsing. This program allows you to manage your to-do list by adding tasks, marking tasks as done, and clearing the entire list.

  

## Features

  

- Add new tasks to the to-do list

- Mark tasks as done by index

- Clear the entire to-do list

  

## Usage

  

### Add a Task

  

```bash

./todo-rust  --add  "Buy groceries"

```

  

### Mark a Task as Done

  

```bash

./todo-rust  --tick  2

```

  

### Clear the To-Do List

  

```bash

./todo-rust  --clear

```

  

## How to Run

  

1. Clone the repository:

  

```bash

git  clone  https://github.com/VimalanS369/todo-rust.git

cd  todo-rust

```

  

2. Build and run the program:

  

```bash

cargo  build  --release

./target/release/todo-rust  --add  "Buy groceries"

```

  

3. Explore additional functionalities using the provided command-line options.

  

## Contributing

  

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

  

## License

  

This project is licensed under the [MIT License](LICENSE).