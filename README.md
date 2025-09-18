# ASIMOV xAI Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-xai-module)](https://crates.io/crates/asimov-xai-module)
[![Documentation](https://docs.rs/asimov-xai-module/badge.svg)](https://docs.rs/asimov-xai-module)

[ASIMOV] xAI module.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with [ASIMOV CLI]

```bash
asimov module install xai -v
```

### Installation from Source Code

```bash
cargo install asimov-xai-module
```

## 👉 Examples

```bash
asimov-xai-prompter
```

## ⚙ Configuration

Provide an API key either by module configuration

```bash
asimov module config xai
```

Or through environment variables

```bash
export XAI_API_KEY="..."
```

### Optional configuration

| Name       | Environment Variable | Default            |
| ---------- | -------------------- | ------------------ |
| `endpoint` | `XAI_API_ENDPOINT`   | `https://api.x.ai` |
| `model`    | `XAI_MODEL`          | `grok-3-mini`      |

## 📚 Reference

### Prompt

```bash
echo "Why is the sky blue?" | asimov-xai-prompter
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-xai-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-xai-module&text=asimov-xai-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-xai-module&title=asimov-xai-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-xai-module&t=asimov-xai-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-xai-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-xai-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
