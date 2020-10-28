# Microgrep

`microgrep` a simple and short scoped Rust implementation of the popular tool `grep`.

## Usage

To look for a word match in any file, enter the following command -

```sh
microgrep essay.txt bird
```

To look for a word match in all the files _inside_ a folder, enter the following command -

```sh
microgrep essayfolder bird
```

To use Regex (PERL flavoured Regex), you'll have to pass the `--regex` flag. here's an example -

```sh
microgrep essay.txt \d\w --regex
```

You can also mix match all these features. Here's an example -

```sh
microgrep essayfolder \d\W --regex
```

And as always if any of your input arguments include a space, you'll have to wrap it in quotes. Here's an example -

```sh
microgrep "my essay folder" "Rust is cool"
```

## Development Process

Firstly ensure that you have all the [Rust related toolchain]() installed. After that enter the directory and run `cargo build`. That should download and install the dependencies. After that you can start developing the tool ;)

## Optional Tooling

Optionally you can install the `watch` tool and use it to run the compiler to compile and run your code when you save a file. To do so, run the following command -

```bash
cargo install watch
```

And then run the following command -

```sh
cargo watch -c -x 'run -- path search -regex'
```
