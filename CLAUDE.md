# Claude Code Instructions for Life Game WASM

## Project Overview
This is a high-performance Conway's Game of Life implementation using Rust/WebAssembly and TypeScript.

## Build Commands
- `bun run build` - Build the entire project (WASM + TypeScript + Vite)
- `bun run build:wasm` - Build only the WASM module
- `bun run build:ts` - Compile TypeScript
- `bun run dev` - Start development server

## Code Quality Commands
Always run these before considering a task complete:
- `bun run typecheck` - Check TypeScript types
- `bun run lint` - Run oxlint
- `cargo clippy -- -D warnings` - Run Rust linter
- `cargo fmt -- --check` - Check Rust formatting

## Test Commands
- `cargo test` - Run Rust tests
- `bun run test:ts` - Run TypeScript unit tests
- `bun run test` - Run all tests (Rust + WASM + TypeScript)

## Project Structure
- `/src` - TypeScript frontend code
- `/src/*.rs` - Rust/WASM source files
- `/pkg` - Generated WASM output (do not edit)
- `/dist` - Built application (do not edit)

## Important Notes
- The WASM module name is `life_game_wasm` (not `rust_webpack_template`)
- Always use proper error handling in Rust (Result types)
- Write comments in Japanese for implementation code (`src/*.rs`, `src/*.ts`),
  matching the existing style. Test files (`tests/*.rs`, `src/*.test.ts`) may
  use English comments, following their existing convention.
- Follow existing code patterns and conventions