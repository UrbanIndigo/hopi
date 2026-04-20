# hopi 🐇

Stop clicking through Studio's Creations dashboard. Type a few letters, pick
from the fuzzy list, and land in the place you wanted — all in under a
second.

`hopi` reads your Studio login, pulls every experience you can edit (yours,
groups you can manage, and Team-Create invites), and opens whichever one you
point at. No sync step. No API keys. No waiting for that group dropdown to
scroll.

## Install

Pick whichever you like:

```sh
# Homebrew (macOS / Linux)
brew install urbanindigo/tap/hopi

# Cargo (any platform with Rust)
cargo install hopi

# Shell one-liner (macOS / Linux)
curl -LsSf https://github.com/UrbanIndigo/hopi/releases/latest/download/hopi-installer.sh | sh

# PowerShell one-liner (Windows)
powershell -c "irm https://github.com/UrbanIndigo/hopi/releases/latest/download/hopi-installer.ps1 | iex"
```

You need Roblox Studio installed and logged in on the same machine — `hopi`
borrows its session from there, so there's nothing else to configure.

## Usage

Just run it:

```
hopi
```

You'll get a three-step picker: *My Experiences / Shared with me / Group
Experiences* → (maybe a group) → the place itself. Arrow keys, fuzzy
matching, enter to launch.

Know where you're going? Skip straight to it:

```sh
hopi me                    # your own experiences
hopi me tycoon             # opens directly if only one place matches

hopi shared                # places other people have shared with you
hopi shared troll

hopi <group>               # any unknown first arg is a fuzzy group-name match
hopi <group> <place>       # straight into that place if exactly one matches
```

Go hop somewhere. 🐇
