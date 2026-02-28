# game-architecting

Small Rust ECS prototype using `legion`.

## What this project was doing

This codebase is a sandbox for trying Legion ECS patterns:

- Components in `src/components.rs`: `Position`, `Calcium`, `Greenness`, `GravityImmune`
- Systems in `src/systems.rs`:
  - `debug_print_entities` (prints positions)
  - `debug_print_greenness` (prints greenness values)
  - `apply_gravity` (increments `y` for entities without `GravityImmune`)
  - `become_more_green` (increments greenness)
- Scheduler wiring in `src/state.rs`
- A simple game loop in `src/main.rs` that spawns test entities and runs systems continuously

## When this was done

- Only recorded commit: `a67ebf9` (`trying legion`)
- Commit timestamp: **2023-10-01 07:36:21 -0500**
- `src/systems.rs` last modified timestamp: **2023-10-01 07:34:58 -0500**

## Collaboration

- Commit author history currently shows only: `wegfawefgawefg <668es218pur@gmail.com>` (1 commit)
- `a67ebf9` includes a co-author trailer: `Co-authored-by: Bergeronimo <Bergeronimo@users.noreply.github.com>`
- Bergeronimo does **not** currently have any direct authored commits in this repository history

## Upstream repository

Git remote is configured and reachable:

- Remote name: `origin`
- URL: `https://github.com/wegfawefgawefg/game-architecting.git`
- Local branch `master` tracks `origin/master` (currently up to date at commit `a67ebf9`)
