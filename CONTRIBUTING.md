# LazyDJ Contribution Guide

Version: 1.0

---

# Purpose

This document defines how work is performed within the LazyDJ repository.

All contributors, human or AI, must follow these rules.

---

# Project Governance

The following documents are authoritative:

1. ARCHITECTURE.md
2. ROADMAP.md
3. AGENT.md
4. REVIEWER.md
5. TECH_STACK.md
6. DECISIONS.md
7. CONTRIBUTING.md
8. PLAN.md

If a conflict exists:

Higher documents override lower documents.

---

# Development Philosophy

Prioritize:

Correctness

over

Speed

Prioritize:

Maintainability

over

Convenience

Prioritize:

Long-term architecture

over

Short-term implementation

---

# Workflow

Every task must follow:

Analyze

Plan

Implement

Validate

Report

No phase may be skipped.

---

# Scope Control

Implement only the requested task.

Do not implement future roadmap phases.

Do not add features outside current scope.

Do not perform speculative development.

---

# Architecture Changes

Any architectural modification requires:

1. Justification
2. Impact analysis
3. Architecture review
4. Documentation update

Required updates:

ARCHITECTURE.md

DECISIONS.md

if applicable.

---

# Technology Changes

Changing technology choices requires:

Justification

Risk analysis

Migration plan

Documentation update

Required updates:

TECH_STACK.md

DECISIONS.md

---

# Dependency Policy

Before adding dependencies:

Evaluate:

* necessity
* maintenance status
* security
* ecosystem adoption

Prefer existing project dependencies whenever possible.

Avoid dependency proliferation.

---

# Code Quality Standards

Required:

cargo fmt

cargo clippy

cargo test

must pass before task completion.

---

# Error Handling

Forbidden:

unwrap()

expect()

panic!()

except inside tests.

Use:

Result<T,E>

typed errors

thiserror

---

# Testing Requirements

Every feature must include validation.

Minimum:

Unit Tests

Critical systems require:

Integration Tests

Property Tests

Benchmarks

where appropriate.

---

# Documentation Requirements

Update documentation when introducing:

New crates

New modules

New architecture

New workflows

New technologies

Documentation is part of the implementation.

---

# Commit Guidelines

Commit messages should be concise.

Format:

type(scope): description

Examples:

feat(audio): add wav decoder

fix(mixer): prevent crossfader overflow

docs(architecture): update event flow

refactor(core): simplify state management

---

# Pull Request Requirements

Every change must provide:

Summary

Motivation

Files Modified

Architecture Impact

Dependencies Added

Tests Added

Validation Performed

Known Limitations

---

# Review Checklist

Before approval verify:

Architecture respected

Roadmap respected

No hidden coupling

No unnecessary dependencies

Tests pass

Documentation updated

No critical technical debt introduced

---

# Definition Of Done

A task is complete only if:

Code implemented

Code validated

Tests pass

Documentation updated

Architecture preserved

Reviewer approval obtained

---

# Forbidden Practices

Do not:

Implement multiple roadmap phases at once

Bypass architecture

Introduce global mutable state

Perform large rewrites without justification

Add unapproved technologies

Ignore validation steps

---

# Long-Term Rule

Every contribution should leave the repository in a better state than it was before.
