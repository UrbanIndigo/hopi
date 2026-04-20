# hop 🐇

Stop clicking through Studio's Creations dashboard. Type a few letters, pick
from the fuzzy list, and land in the place you wanted — all in under a
second.

`hop` reads your Studio login, pulls every experience you can edit (yours,
groups you can manage, and Team-Create invites), and opens whichever one you
point at. No sync step. No API keys. No waiting for that group dropdown to
scroll.

## Install

```sh
git clone https://github.com/UrbanIndigo/hop
cargo install --path hop
```

That's it. You need Roblox Studio installed and logged in on the same
machine — `hop` borrows its session from Studio so there's nothing else to
configure. macOS and Windows both work.

## Usage

Just run it:

```
hop
```

You'll get a three-step picker: *My Experiences / Shared with me / Group
Experiences* → (maybe a group) → the place itself. Arrow keys, fuzzy
matching, enter to launch.

Know where you're going? Skip straight to it:

```sh
hop me                    # your own experiences
hop me tycoon             # opens directly if only one place matches

hop shared                # places other people have shared with you
hop shared troll

hop <group>               # any unknown first arg is a fuzzy group-name match
hop <group> <place>       # straight into that place if exactly one matches
```

Go hop somewhere. 🐇
