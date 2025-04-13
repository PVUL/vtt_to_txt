# 🎧 vtt-to-txt

A simple and fast Rust CLI tool to convert `.vtt` subtitle files into clean plain text for downstream use (e.g., training RAG models).

---

## ✨ Features

- Parses `.vtt` (WebVTT subtitle) files
- Strips timestamps, notes, and metadata
- Outputs a clean `.txt` file
- Optional argument for custom output path
- Lightweight, no dependencies

---

## 🛠 Installation

```
git clone https://github.com/PVUL/vtt_to_txt.git
cd vtt_to_txt
cargo build --release
```

🚀 Usage
```
cargo run -- path/to/myfile.vtt
```
This will create a clean text file.

Optionally, specify an output file:

```bash
cargo run -- path/to/myfile.vtt path/to/cleaned.txt
```

📂 Example
Input: `transcript.vtt`

```
WEBVTT

00:00:01.000 --> 00:00:03.000
Welcome to the show.

00:00:03.500 --> 00:00:06.000
Today we talk about AI and the future.
```

Output: `transcript.txt`

```
Welcome to the show. Today we talk about AI and the future.
```