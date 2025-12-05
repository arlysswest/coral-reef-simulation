# AUTHORSHIP INFORMATION

Name: Arlyss West
Class: CS-423 Rust Programming
Licence: MIT + Apache 2.0
Date: 12/03/2025

# ABOUT THE PROGRAM

(fill me in)

## PROGRAM DESIGN

the full program design is located in "project-plan.pdf"

### Texed-based Version

(fill me in)

### Visual Version

(fill me in)

## RUNNING THE PROGRAM

### RUN THE PROGRAM:

    cargo run

### BEVY VERSION

    bevy 0.14

## TOOLS USED

# Game Engine

    Bevy version 0.14

# Coral Image

https://img.freepik.com/premium-psd/coral-isolated-transparent-background-png-psd_888962-433.jpg

## BRANCHES

1. main -> contains finsihed version or current best version
2. visual-version -> contains current version of the visual version
3. text-based -> contains the finished text based version

### PROCESS

## What Worked

(fill me in)

## What didn't work

# Trying to add all of the files to the repository

I needed to either use a .gitignore or only add specific files instead of a general "git add ." without a .gitignore. I tried to push files that were too large and unecessary to add to the repository.

# Not making a back up version

When I tried to push the first version of the visual version with the too large of files, i messed up my git history. I lost the version in the process and had to revert back to the working version I had on githb before that. This could have been avoided had I made a backup version.

## What lessens were learned

(fill me in)

# What Could Be Added Or Improved

(fill me in)

### RESEARCH

## Research tools

- Chat GPT
- https://bevy.org/learn/quick-start/introduction/

## Debugging

- copy and pasted errors into chatgpt to better understand what the errors mean when I am unfamiliar with an error
- git revision control syntax for merging to resolve merge issues

## Simulation Design

# Color Codes

used chatgpt to pick color codes:

- "color code for sea blue?"
- "color code for coral orange?"
- "What colors would look good in combination with my current colors?"

## Coral Reef Research

- ai suggested changes to ph, coral cover %, algea cover %, and water temperature when a problem or tool is applied based off what a realistic change in the statistics would be
  example: coral cover +5% when articicial substrate is applied
- ai suggested starting statistics based off of what realistic statistics would be
  example: coral: 35%, algae: 10%, temp: 27.0, ph: 8.1
- Ai suggested problems and tools that would be realistic for a coral reef
- Ai definitions for problems and rewards that are accurate for the problems and rewards I chose

## Game Engine Research

# Options

- Bevy
- Fyrox (formerly RG3D)
- Piston
- Amethist

# Choice : Bevy

- PROS:

* Modern ECS -> clean, fast, simple to write.
* Pure Rust -> safe, fast, great tooling.
* All-in-one engine (2D/3D, UI, input, audio).
* Cross-platform including Web (WASM).
* Active, fast-moving community.
* STRONG DOCUMENTATION WITH A LOT OF TUTORIALS

- CONS:

* Frequent breaking changes (not fully stable).
* No visual/scene editor yet (code-only workflow).
* 3D not AAA-level; some features still basic.
* WASM quirks (not all crates supported).
* You must optimize ECS queries yourself.
* can be heavy and have a long loading time.
