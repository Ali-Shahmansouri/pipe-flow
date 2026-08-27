# 🎮 Pipe Flow

A little terminal puzzle game written in Rust

This is a hobby challenge project I built while learning Rust and scratching the surface of terminal UI development.

There's the idea:

> Rotate the pipes, connect the source to destination, and let the water flow!

The project is not finished and there's always work in progress, occasionally improving it.

---

## Game Concept

1. There is a grid (rectangular).

2. A flow is coming from a **start cell** to an **end cell**.

3. There are 3 cell types for determining the flow:
  a. **Straight** (down/right, or left/up, and vice versa)
  b. **LShaped** (horizontal, or vertical)
  c. **Block** (completely blocks the flow)

4. The player can **rotate any cell except the Block** cell type, the cell rotates by 90 degrees each time you click on it.

5. A cell can be **Fixed** that prevents the player from rotating it, meaning that **it is frozen in the rotation and the shape** it is in when generated.

6. The board must be randomly generated with a non-repeating valid route from "start cell" to "end cell" as the main solution path, even though there may be alternative valid ways to solving the puzzle.

7. Each cell should visually display that it has a flow, or it is fixed, and more (either by changing its color or applying some overlay or any other way).

---

## How it works?

The game is split into a few different ideas.

### Cells

There are currently a few different cell types:

- `LShapedCell`: A diagonal pipe looking like the letter `L`
- `StraightCell`: An orthogonal pipe looking like a straight line
- `BlockCell`: A square looking flow-blocker obstacle

Each cell knows about its current pipe connections.

For Example

```
Up + Right

  │
  │
  └──
```

Rotating the cell changes its Rotation, which changes its connections.

### Connections

Pipes don't really care about how they look.

They care about which directions they connect to:

```
    Up
     │
Left─┼─Right
     │
    Down
```

So a cell can describe itself with something like:

```rs
Connections::new(
    true,  // up
    false, // down
    true,  // right
    false, // left
);
```

The visual representation is then generated from those connections.

This separation turned out to be pretty useful.

### Flow Detection

Once the player rotates a pipe, the board recalculates which cells are connected
to the source.

The current implementation uses a simple depth-first search (DFS).

A neighbouring cell is considered connected only when both sides agree.

So a pipe pointing right at a pipe that doesn't point left does not create a
connection.

### Random Board Generation

The board can generate random puzzles with arbitrary dimensions.

The generator works roughly like this:

1. Choose Start (A random edge + a random Position on the edge)

2. Choose Destination (A random edge + a random Position on the edge)

3. Generate a random path

4. Turn path into pipe connections

5. Turn pipe connections into Cells

6. Fill Everything else with random Cells

7. Scramble and rotate pipes

### Terminal UI

The game uses [ratatui](https://ratatui.rs/) for rendering and
[crossterm](https://github.com/crossterm-rs/crossterm) for keyboard input.

The pipes themselves aren't images.

They're made from Unicode box-drawing characters:

```
│ ─ ┌ ┐ └ ┘
```

and the flowing version uses heavier characters indicating water flow:

```
║ ═ ╔ ╗ ╚ ╝
```

---

## Running the Project
You'll need Rust installed.

```bash
cargo run
```

Hopefully a little pipe puzzle appears in your terminal. 😁🤞

---

## What's next?

There are still plenty of things I'd like to add.

### Gameplay
- Detect when the destination is reached
- Show a proper "You Win!" state
- Animate flowing water
- Add different difficulty levels
- Add a move counter
- Add a timer
- Generate more interesting filler cells
- Make generated puzzles more challenging

### Rendering
- Improve pipe graphics
- Better source/destination indicators
- Water-flow animation
- Better terminal layouts
- Maybe add some simple UI around the board

---

## Built with Rust

**Have Fun CONNECTING PIPES!**
