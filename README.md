# hop

Open Roblox Studio places from the command line. Skip the Studio launcher,
the Creations dashboard, the group dropdown, and the place grid — just fuzzy-
pick and go.

## Install

```
cargo install --path .
```

This puts `hop` in `~/.cargo/bin`. You'll need Roblox Studio installed and
logged in on the same machine — `hop` reads the `.ROBLOSECURITY` cookie
straight out of Studio's local config (`~/Library/Preferences/com.roblox.RobloxStudioBrowser.plist`
on macOS, `HKCU\SOFTWARE\Roblox\RobloxStudioBrowser\roblox.com` on Windows)
via the [`rbx_cookie`](https://crates.io/crates/rbx_cookie) crate. No
passwords, API keys, or manual setup.

## Usage

Interactive (3-step picker):

```
hop
```

Step 1 offers `My Experiences`, `Shared with me`, `Group Experiences`. Step 2
is the place list for Me/Shared, or a group list for Groups. Step 3 (Groups
only) is the place list for the picked group.

Shortcuts:

```
hop me                    # pick from your own experiences
hop me tycoon             # direct open if unique, picker if ambiguous
hop shared                # pick from Team-Create-shared experiences
hop shared troll

hop bluestar              # fuzzy-match a group, then pick a place
hop bluestar troll        # direct open if unique within the group
```

Any first argument that isn't `me` / `self` / `mine` / `shared` is treated as
a fuzzy group-name query.

