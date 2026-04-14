# Logan

Logan is an intuitive, fast, and elegant Terminal User Interface (TUI) application written in Rust. It helps you parse, search, and visually filter log files right from your terminal without opening a bulky GUI!

## 🚀 Features

- **TUI Interface:** A beautiful ratatui-driven interface designed for reading logs effortlessly.
- **Multiple Parsers:** Automatically identifies and parses both plain-text and JSON-formatted log entries.
- **Filtering:** A sidebar with filter options by severity levels (`Trace`, `Debug`, `Info`, `Warn`, `Error`, `Unknown`).
- **Interactive Search:** Press `/` to start typing and perform real-time text-matching across all log entries.

## 📦 Installation

### Quick install (Unix/Linux)
Install the latest version automatically using our script. It detects your operating system, downloads the correct binary from our Releases, and installs it securely.

```shell
curl -fsSL https://raw.githubusercontent.com/Ogwenya/Logan/main/install.sh | sh
```

*(Alternatively, you can manually download the latest release from the [release page](https://github.com/Ogwenya/Logan/releases) or build it with `cargo`)*

## 🛠 Usage

To use Logan to inspect a log file, simply pass the path of your file as an argument:

```bash
logan <path_to_your_log_file>
```

**Example:**
```bash
logan ./example.log
```

### Key Bindings & Mouse Usage

| Key/Action                   | Description                                                |
|------------------------------|------------------------------------------------------------|
| `Up` / `Down` Arrow keys     | Scroll up and down inside the terminal window logs feed.   |
| `Click` on Sidebar Levels    | Filter the logs feed to selectively show a specific level. |
| `Click Reset` (Sidebar)      | Show all log levels again.                                 |
| `/`                          | Enter **Search Mode** to dynamically filter message text.  |
| `Esc`/`Enter` in Search Mode | Escape the active search query box layout block.           |
| `q`                          | Quit the application.                                      |


