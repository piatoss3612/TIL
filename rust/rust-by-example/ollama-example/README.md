# Ollama

## Installation

> on linux

```bash
$ curl https://ollama.ai/install.sh | sh
```

## Install Model and Run

```bash
$ ollama run (model name)
```

### Example

```bash
$ ollama run llama2
```

## Run example

```bash
$ cargo run -q --example 01-simple
->> res Rust by light years!
```

```bash
$ cargo run -q --example 02-context

Prompt: Why the sky is red? (be concise)

The sky can appear red during sunrise and sunset due to a phenomenon called Ray
leigh scattering, where shorter wavelengths of light are scattered more than
 longer wavelengths. This causes the blue and violet light to be blocked, leaving
 mostly red and orange light to reach our eyes, giving the sky its reddish hue.

Prompt: What was my first question?

Your first question was "Why the sky is red?"
```

## References

- [Github](https://github.com/jeremychone-channel/rust-xp-ollama)