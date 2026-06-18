# DECISIONS

Version: 1.0
Date: 2026-06-18

This document records the explicit architectural and technology decisions that implement the constraints and choices expressed in ARCHITECTURE.md, TECH_STACK.md, ROADMAP.md, AGENT.md, PLAN.md and CONTRIBUTING.md. Changes to these decisions require justification, impact analysis, and documentation updates.

---

DEC-001: Language & Workspace
- Decision: Use Rust (>= 1.96) with a Cargo workspace.
- Rationale: Safety, performance, cross-platform support, and ecosystem fit for low-latency audio.
- Implications: CI must pin Rust toolchain; crates follow single-responsibility boundaries.
- Related: TECH_STACK.md, PLAN.md

---

DEC-002: Async & Concurrency
- Decision: Use Tokio for async tasks; use Rayon for CPU-bound parallel work where appropriate.
- Rationale: Tokio for IO/concurrency, Rayon for deterministic CPU-parallel workloads (analysis, waveform generation).
- Implications: Background workers run on Tokio/Rayon; audio thread remains single-purpose and non-blocking.
- Related: ARCHITECTURE.md, TECH_STACK.md

---

DEC-003: Messaging & Event Bus
- Decision: Use an event-driven core with flume as the preferred messaging primitive between modules.
- Rationale: Decouples modules, preserves single responsibility, supports the Event Bus pattern.
- Implications: Avoid direct cross-crate calls; prefer event/messages for coordination.
- Related: ARCHITECTURE.md

---

DEC-004: Audio I/O and Decoding
- Decision: Use cpal for audio output and symphonia for decoding (WAV, MP3, FLAC, OGG). Use hound where WAV-specific processing is needed.
- Rationale: Portable audio IO (cpal) and pure-Rust decoding (symphonia) align with low-latency goals.
- Implications: Audio thread interacts only with pre-decoded or lock-free buffers; decoding and disk IO occur off the audio thread.
- Related: TECH_STACK.md, ARCHITECTURE.md

---

DEC-005: DSP and Benchmarking
- Decision: Use rustfft/realfft/rubato for DSP primitives; use Criterion for benchmarking critical DSP paths.
- Rationale: Established crates for FFT, resampling and deterministic measurement tooling.
- Related: TECH_STACK.md

---

DEC-006: Database & Persistence
- Decision: Use SQLite with sqlx for persistence; isolate DB access to a dedicated thread or background task.
- Rationale: Lightweight embedded DB, transaction support, fits offline-first design.
- Implications: Audio thread must never perform DB access.
- Related: ARCHITECTURE.md, TECH_STACK.md

---

DEC-007: Logging & Observability
- Decision: Use tracing and tracing-subscriber for structured logs and observability.
- Rationale: Structured logging and leveled diagnostics required for debugging runtime audio issues.
- Related: TECH_STACK.md

---

DEC-008: Downloads & Stems
- Decision: Use yt-dlp for downloads; use Demucs for stems separation (in background workers, not audio thread).
- Rationale: Best-of-breed tools for the job; run as external processes in worker contexts with validation and sandboxing.
- Implications: Validate inputs; cache outputs; do not invoke external processes from audio paths.
- Related: ARCHITECTURE.md, ROADMAP.md

---

DEC-009: GUI & Plugin Model
- Decision: Use egui/eframe for GUI; plugin system allows Effects, Visualizers, AI Tools, Hardware Drivers; plugins must register and run with limited permissions.
- Rationale: Fast iteration UI with Rust-friendly stacks; plugins enable extensibility without recompiling core.
- Implications: Plugins may run in sandboxed/background contexts; plugin lifecycle: load → register → initialize → execute → unload.
- Related: ARCHITECTURE.md, TECH_STACK.md

---

DEC-010: Audio Thread Constraints
- Decision: Audio thread must never block, perform IO, access SQLite, access network, or execute external processes.
- Rationale: Preserve low latency and real-time guarantees.
- Enforcement: Design API boundaries, code reviews, and tests must ensure no accidental IO in audio paths.
- Related: ARCHITECTURE.md, AGENT.md, REVIEWER.md

---

DEC-011: Error Handling
- Decision: Use typed Result<T,E> and thiserror; forbid unwrap(), expect(), and panic!() in production code.
- Rationale: Predictable error propagation and maintainability.
- Related: AGENT.md, CONTRIBUTING.md

---

DEC-012: Testing & Validation
- Decision: Require unit tests for all features; use proptest for critical modules and criterion for DSP benchmarks.
- Rationale: Validation-first culture to avoid regressions and ensure audio stability.
- Related: AGENT.md, CONTRIBUTING.md

---

How to change a decision
- Any change to these decisions requires: justification, impact analysis, architecture review (Reviewer), and updates to DECISIONS.md and related docs (ARCHITECTURE.md / TECH_STACK.md) per CONTRIBUTING.md.

---

Notes
- This file is synthesized from the existing project docs (ARCHITECTURE.md, TECH_STACK.md, ROADMAP.md, AGENT.md, PLAN.md, CONTRIBUTING.md). If any additional explicit decision is required, open an ADR entry here with rationale and consequences.
