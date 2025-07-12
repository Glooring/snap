# 📦 Snap (The Rust Edition)

**A truly native, blazing-fast, and portable snapshot tool for Windows developers. Create efficient, versioned backups of your project folders with full support for hidden file attributes.**

`snap` is a command-line utility that provides a simple way to capture point-in-time states of your projects. It uses the power and reliability of Git for its core operations but provides a simplified, focused workflow. It's perfect for quick, local backups before a major refactor, for archiving project milestones, or for any situation where you need a reliable "undo" button for your entire directory.

---

## From Node.js to Native: The Quest for Speed

This project began its life as a Node.js application, packaged into an `.exe` for convenience. While functional, it had a noticeable startup delay inherent to the Node.js runtime. For a tool designed to be a quick, seamless part of a developer's workflow, this was a critical friction point.

The desire for **instantaneous, native-level performance** led to a complete rewrite in **Rust**. By compiling directly to a native binary, `snap` now launches as fast as `git` itself, eliminating all runtime overhead. This new version retains 100% of the original's features while providing the speed and responsiveness a command-line power tool deserves.

---

## 📋 Table of Contents

*   [Key Features](#-key-features)
*   [How It Works](#-how-it-works)
*   [Prerequisites](#-prerequisites)
*   [Installation](#-installation)
*   [Usage](#-usage)
    *   [`snap init`](#1-snap-init)
    *   [`snap new <label> [description...]`](#2-snap-new-label-description)
    *   [`snap list [limit]`](#3-snap-list-limit)
    *   [`snap diff <snapshot-A> <snapshot-B>`](#4-snap-diff-snapshot-a-snapshot-b)
    *   [`snap restore [id_or_label]`](#5-snap-restore-id_or_label)
    *   [`snap delete [id_or_label]`](#6-snap-delete-id_or_label)
    *   [`snap edit [id_or_label]`](#7-snap-edit-id_or_label)
    *   [`snap update`](#8-snap-update)
    *   [`snap options`](#9-snap-options)
*   [Building from Source](#-building-from-source)
*   [Troubleshooting](#️-troubleshooting)
*   [Contributing](#-contributing)
*   [License](#-license)

---

## ✨ Key Features

*   **🚀 Truly Native Performance**: Rewritten in Rust for instantaneous startup and execution. Feels as fast and responsive as `git` itself.
*   ** Fully Portable**: The entire snapshot history, including all metadata for hidden files and empty directories, is stored *inside* the project's `.git` directory. Cloning or zipping the project folder transfers everything.
*   ** dependable Git Core**: Leverages the rock-solid foundation of `git.exe` for all file storage and versioning. Snapshots are lightweight, annotated Git tags.
*   **⚡️ Efficient Storage**: Benefits from Git's mature Content-Addressable Storage model. Identical files across hundreds of snapshots consume the space of just one file.
*   **🎯 Smart Restores**: `snap restore` intelligently checks for uncommitted local changes and prompts you to prevent accidental data loss. It also restores your project files and synchronizes hidden file attributes and empty directories.
*   **🪟 Windows Native**: Fully aware of Windows file attributes. It correctly preserves and restores the **Hidden** status of files and folders using its robust internal metadata system. It also tracks and restores empty directories, which Git normally ignores.
*   **✍️ Rich Metadata**: Snapshots are tagged with a simple `label` and a more detailed `description`, which are stored directly in Git's annotated tags.
*   **💻 User-Friendly CLI**: Features an interactive, arrow-key menu, human-readable timestamps, a compact list view, and an **`(active)`** marker to show you exactly which version your project is on.
*   **🚫 Zero Dependencies**: The compiled `snap.exe` is a single, standalone binary. No need for Node.js, Rust, or any other runtime on the user's machine.

---

## ⚙️ How It Works

`snap` manages your project as a standard Git repository but uses a simple, robust interaction model. It is now **fully self-contained**.

1.  **The Project Repository (`.git`)**
    When you run `snap init`, your project folder becomes a standard Git repository. `snap` then acts as a user-friendly wrapper around `git.exe`.

2.  **The Internal Metadata Store**
    Because Git doesn't natively track the "Hidden" attribute on Windows or empty directories, `snap` captures this information. Instead of using an external folder, it now does the following:
    *   It bundles the metadata (lists of hidden paths and empty directories) into a JSON string.
    *   It stores this JSON string as a native Git "blob" object inside your project's `.git/objects` directory using the `git hash-object` command.
    *   It then embeds a reference to this blob's hash directly into the annotated Git tag that represents the snapshot.

**Visualized Structure:**

```text
// Your project folder contains the Git repository:
D:/Projects/MyUnityGame/
└── .git/
    └── objects/
        ├── ...
        ├── 8f/729c75...  <-- A Git Blob containing your metadata JSON
        └── ...

// The Git tag for your snapshot links everything together:
- Tag Name: "v1.0"
- Points to Commit: f4e5d6c...
- Tag Message:
  "Initial release.

   Snap-Metadata-Ref: 8f729c75..."
```

This elegant design keeps your project folder clean, standard, and **100% portable**.

---

## 🔧 Prerequisites

*   Windows 10 or later.
*   **Git for Windows** must be installed and accessible in your system's PATH. `snap` relies on `git.exe` for all core operations. You can get it from [git-scm.com](https://git-scm.com/download/win).

---

## 📥 Installation

You can either download a pre-compiled binary or build it from the source yourself.

1.  **Download the Release**
    *   Go to the [Releases](https://github.com/your-username/snap/releases) page of this repository.
    *   Download the latest `snap.exe` file.

2.  **Place it in your PATH**
    *   Move `snap.exe` to a permanent location on your system (e.g., `D:\Tools\`).
    *   Add this directory (`D:\Tools`) to your Windows PATH environment variable. This allows you to run the `snap` command from any terminal.

3.  **Verify**
    Open a **new** terminal window (so it loads the new PATH). Navigate to a project you want to back up and run `snap init`. It should initialize the repository instantly.

---

## 🚀 Usage

All commands are run from within your project's directory.

### 1. `snap init`
Initializes the current folder as a `snap` repository. **Must be run once per project.** This command is now non-interactive and much simpler.

```cmd
D:\Projects\my-app>snap init
[snap] Initialized empty snap repository in D:\Projects\my-app
```

### 2. `snap new <label> [description...]`
Creates a snapshot of the current project state.

```cmd
D:\Projects\my-app>snap new v1.0 "Initial release"

[snap] Step 1/4: Scanning for metadata (hidden files, empty dirs)...
[snap] Step 2/4: Staging all files...
[snap] Step 3/4: Creating the commit...
[snap] Step 4/4: Creating the annotated snapshot tag...

[snap] New snapshot created: [a1b2c3d] v1.0
```

### 3. `snap list` [limit]
Lists all available snapshots in a compact view, showing the active one.

```cmd
D:\Projects\my-app>snap list 2

[snap] Snapshots for "my-app":

  Label           Description                   Timestamp
  --------------  ----------------------------  ----------------
  v1.1            Second public release         2025-06-16 09:15   (active)
  v1.0-hotfix     A quick fix for the release   2025-06-15 18:00
  ... and 1 more. Use 'snap list all' to see all snapshots.
```
If [limit] is omitted, it uses the default value from your configuration (changeable via snap options). Use snap list all to view all snapshots regardless of the configured limit.

### 4. `snap diff <snapshot-A> <snapshot-B>`
Compares two snapshots and shows a list of changes.

```cmd
D:\Projects\my-app>snap diff v1.0 v1.1

[snap] Comparing snapshots a1b2c3d ("v1.0") ➜ f4e5d6c ("v1.1"):

  + src/auth/new-logic.js
  - config/old-settings.json
  ~ src/app.js
  ! .env (visibility changed)
  + assets/sounds/ (empty directory)

[snap] Summary: 1 added, 1 deleted, 1 modified, 1 visibility change, 1 empty dir added
```

### 5. `snap restore [id_or_label]`
Restores the project to a previous state. If run without arguments, it displays an interactive menu.

```cmd
D:\Projects\my-app>snap restore v1.0

[snap] Restoring project files for snapshot "v1.0"...
[snap] Synchronizing metadata...
[snap] Restore complete. Your project is now at the state of this snapshot.
```

### 6. `snap delete [id_or_label]`
Permanently deletes a snapshot. The associated metadata blob is automatically cleaned up by Git's garbage collection later.

```cmd
D:\Projects\my-app>snap delete v1.1-hotfix

[snap] You are about to delete snapshot:
  Label: v1.1-hotfix
? [snap] WARNING: This will permanently delete the snapshot tag. Continue? [y/N] y
[snap] Deleting tag "v1.1-hotfix"...
[snap] Snapshot "v1.1-hotfix" deleted successfully.
```

### 7. `snap edit [id_or_label]`
Edits the label and description of an existing snapshot.

```cmd
D:\Projects\my-app>snap edit v1.0
? Select snapshot to edit: › v1.0
[snap] Editing snapshot "v1.0":
? Enter new label (tag name): v1.0-final
? Enter new description: Final version for initial release
...
[snap] Snapshot successfully updated to "v1.0-final".
```

### 8. `snap update`
Amends the **active** snapshot with the current state of the project.

```cmd
D:\Projects\my-app>snap update

[snap] This command will replace the active snapshot...
  Target Snapshot:
    Label:       v1.1
? [snap] This will amend the commit for snapshot "v1.1". This action is hard to undo. [y/N] y
...
[snap] Update complete. Snapshot "v1.1" now points to new commit [b8c9d0e].
```

### 9. `snap options`
Allows you to configure global UI settings. These are stored in a `.snapconfig` file next to the executable.

```cmd
D:\Projects\my-app>snap options
? Select option to change:
> showIds            - Controls if IDs are shown in lists (current: false)
  confirm_command    - Asks for y/N on destructive actions (current: true)
  orderBy            - Controls the sort order for 'snap list' (current: Timestamp)
  editUpdatesTimestamp - Controls if editing a snapshot updates its timestamp (current: false)
  listLimit          - Sets how many snapshots to show with 'snap list' (current: all)
```
---

## 🏗️ Building from Source

If you want to modify the tool or build it yourself, you'll need the Rust toolchain.

1.  **Install Rust**: If you don't have it, get it from [rustup.rs](https://rustup.rs).
2.  **Clone the Repository**:
    ```cmd
    git clone https://github.com/your-username/snap.git
    cd snap
    ```
3.  **Build the Release Executable**:
    ```cmd
    cargo build --release
    ```
4.  **Find the Executable**: The final `snap.exe` will be in the `target/release/` directory. You can then copy it to a location in your PATH.

---

## 🛠️ Troubleshooting

*   **`Error: Git is not installed or not in your system PATH.`**: `snap` requires `git.exe` to function. Install Git for Windows and ensure its `bin` and `cmd` directories are in your system's PATH.
*   **`'snap' is not recognized...`**: This means the directory containing `snap.exe` was not correctly added to your PATH, or you haven't opened a new terminal window since adding it.
*   **`Error: Not a snap repository...`**: You are trying to run a command (like `list` or `new`) inside a directory that has not been initialized. Run `snap init` first.

---

## 🤝 Contributing

Contributions are welcome!

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/my-new-feature`).
3.  Make your changes to the `.rs` files within the `src/` directory.
4.  Test your changes by running `cargo run -- <command>` (e.g., `cargo run -- list`).
5.  **Rebuild the release executable** by running `cargo build --release`.
6.  Test the new `snap.exe` file thoroughly.
7.  Commit your changes (`git commit -am 'Add some feature'`).
8.  Push to the branch (`git push origin feature/my-new-feature`).
9.  Open a Pull Request.

---

## 📜 License

This project is licensed under the MIT License.