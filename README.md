# AUTHORSHIP INFORMATION

Name: Arlyss West
Class: CS-423 Rust Programming
Licence: MIT + Apache 2.0
Date: 12/03/2025

# ABOUT THE PROGRAM

This project simulates the health of a coral reef ecosystem and models how restoration decisions and environmental stressors affect the reef over time. The core of the simulation revolves around four statistics: coral cover %, algae %, water pH, and temperature, which update dynamically as the user interacts with the system.

Positive actions (restoration tools) improve the reef, while randomly occurring real-world problems (pollution, CO₂ emissions, invasive species, storms, and overfishing) reduce reef health. The goal is to keep the coral reef alive by making informed restoration choices while responding to environmental challenges.

The simulation is implemented first as a text-based version, and optionally as a visual version using the Bevy game engine.

## PROGRAM DESIGN

the full program design is located in "project-plan.pdf"

### Texed-based Version

The text-based version follows a clear turn-based loop. Each turn:

1. Displays current reef statistics
2. Asks the user to choose a restoration tool or quit
3. Updates the reef based on the selected tool
4. Shows a positive message
5. Randomly applies a reef-damaging environmental problem
6. Updates statistics again
7. Repeats until the user quits or coral cover reaches 0%

The text version serves as the minimum viable product (MVP) and forms the logical foundation for the visual version.

### Visual Version

The visual version implements the same simulation logic but adds:

1. A reef visualization that displays coral images based on coral cover %
2. A 3×3 interactive map, where each region is a separate reef cell with its own health
3. Clickable tools in a sidebar
4. A message panel showing recent events
5. Real-time stat updates
6. A clean UI built with Bevy 0.14
7. Transparent coral sprites generated via AI

This version transforms the simulation from a command-line loop into a fully interactive visual experience.

## RUNNING THE PROGRAM

### RUN THE PROGRAM:

    cargo run

### BEVY VERSION

    bevy 0.14

## BRANCHES

1. main -> contains finsihed version or current best version
2. visual-version -> contains current or finsihed version of the visual version
3. text-based -> contains current best version of the program, either visual or text-based

### PROCESS

## What Worked

I was able to fully implement the simulation logic for both the text-based and visual versions.
Important successes:

1. Bevy was effective for creating UI layouts, buttons, images, and real-time updates.
2. Implementing coral visualization based on coral cover % worked well.
3. Adding a 3×3 interactive map was successful and made the project more engaging.
4. Structured state management (Start -> Game -> Game Over) kept the program organized.

## What didn't work

# Trying to add all of the files to the repository

I needed to either use a .gitignore or only add specific files instead of a general "git add ." without a .gitignore. I tried to push files that were too large and unecessary to add to the repository.

# Not making a back up version

When I tried to push the first version of the visual version with the too large of files, i messed up my git history. I lost the version in the process and had to revert back to the working version I had on githb before that. This could have been avoided had I made a backup version.

## What lessons were learned

1. Always add a .gitignore early in the project—especially when using game engines like 2.2. Bevy that generate large build artifacts.
2. Create backup branches before attempting major refactors or adding many assets.
3. ECS programming encourages modularity, but you must carefully manage queries to avoid Bevy borrow conflicts.
4. Rust’s borrow checker catches issues early, but understanding lifetimes and mutable references in ECS systems takes practice.
5. Visual UI programming is iterative: start simple, then refine layout, colors, and features.
6. Transparent PNG/AVIF assets must be tested in Bevy to confirm alpha channels load correctly.

# What Could Be Added Or Improved

1. The map function could be improved. As of right now the design is very basic and doesnt have any detail. The design could be improved to display a coral reef instead of the blue/orange/white squares.

2. A more info option could be added to the visual version. I implemented a more info option to the text-based version that would give more info about the tools, problems, and staistics when selected to inform decisions. This could be added as its own button or as a hover option to the visual version.

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

# AI generated Image of a coral

I couldn't find an open source & free transparent image of a coral to use online. I could have created the code to draw the coral over and over, but I decided to not use that method. Instead I asked AI to "generate an image of an orange coral with a transparent background". This image is in the assets folder with the name "coral-transparent.png"

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

# Choice : Bevy pros & Cons

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

# Bevy Syntax

- From the Bevy website:
  I learned the basics of UI nodes, layout styling, spawning and despawning entities, handling button interactions, and loading assets like images and fonts.

- From ChatGPT:
  I learned how to correctly use transparent images, fix ECS query conflicts with ParamSet, and troubleshoot Bevy errors.
