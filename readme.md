# 🎌 Kurika Meguri

<div align="center"> 

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=flat-square&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![SQLx](https://img.shields.io/badge/SQLx-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/launchbadge/sqlx)
[![Status](https://img.shields.io/badge/Status-In%20Development-orange?style=flat-square)]()
</div>

> A Japanese learning platform built around repetition, active recall, and continuous progress.

**Kurika Meguri** is a Japanese learning web application designed to make learning and practicing Japanese more structured, accessible, and engaging.

---

## 🎀 Why Kurika Meguri?
Learning Japanese is hard.

There are multiple writing systems, unfamiliar characters, vocabulary to memorize, kanji to recognize, and grammar patterns to understand. More importantly, simply studying something once is rarely enough to actually remember it.

At the same time, many polished language-learning applications rely heavily on subscription models. For learners who are just starting out, paying for a recurring subscription can become a barrier.

Kurika Meguri started from a simple idea:

> **What if learning Japanese could be built around a focused repetition cycle without requiring learners to immediately commit to an expensive subscription?**

The goal is not to build another massive language-learning application overnight.

The goal is to build a useful learning experience, one step at a time.

---

## 🔄 The Idea Behind the Name

**Kurika** comes from *kurikaesu* (繰り返す), meaning **to repeat** or **repetition**.

**Meguri** represents a **cycle or continuous loop**.

Together, **Kurika Meguri** represents the idea of repeatedly revisiting knowledge through a continuous learning cycle.

```text
             ┌──────────┐
             │   Learn  │
             └────┬─────┘
                  ↓
             ┌──────────┐
             │ Practice │
             └────┬─────┘
                  ↓
             ┌──────────┐
             │  Review  │
             └────┬─────┘
                  ↓
             ┌──────────┐
             │ Improve  │
             └────┬─────┘
                  │
                  └──────────↺
```

The long-term direction is inspired by **Spaced Repetition Systems (SRS)**, where previously learned material can return at appropriate intervals based on learning history and performance.

---

## 📚 Learning Experience

Kurika Meguri is built around **active recall rather than passive consumption**.

Instead of simply presenting a list of Japanese characters or vocabulary, the platform aims to continuously bring learned material back into practice.

The initial learning experience begins with:

### Hiragana

- Learn characters and their romaji
- Practice character recognition
- Answer randomized exercises
- Receive immediate feedback
- Revisit previously learned material
- Gradually build stronger recognition

The first version is intentionally small.

A solid learning loop comes before a large feature set.

---

## 🗾 Long-Term Direction

The platform is intended to gradually expand across different areas of Japanese:

```text
Hiragana
    ↓
Katakana
    ↓
Vocabulary
    ↓
Kanji
    ↓
Grammar
```

These areas are intended to become part of a connected learning system rather than a collection of unrelated features.

---

## ⚙️ Backend

- **Rust** — Core backend language
- **Axum** — HTTP framework
- **Tokio** — Asynchronous runtime
- **SQLx** — Database access
- **PostgreSQL** — Relational database
- **Serde** — Serialization and deserialization

---

## 🏗️ Engineering

Kurika Meguri is being developed as a **real product**, not simply as a portfolio CRUD application.

As the product evolves, the backend will address production concerns such as:

- Authentication & authorization
- Database design and migrations
- Data integrity
- Testing
- Error handling
- Security
- Rate limiting
- Logging
- Observability
- Deployment
- Backup & recovery
- Reliability and maintainability

The intention is to build something that can eventually serve real users and handle real-world requirements.

---

## 🧩 Architecture

The backend follows a modular structure with clear responsibilities:

```text
                    HTTP Request
                         │
                         ▼
                      Routes
                         │
                         ▼
                      Handlers
                         │
                         ▼
                      Services
                         │
                         ▼
                       SQLx
                         │
                         ▼
                    PostgreSQL
```

The architecture is intentionally kept simple during the early stages and will evolve alongside the actual requirements of the product.

---

## 🎯 Product Philosophy

### Start Small

Solve one clear learning problem before trying to solve everything.

### Learn by Doing

Prioritize active recall and practice over passive consumption.

### Repeat

Previously learned knowledge should return to the learning cycle.

### Build for Real Users

Product decisions should eventually be driven by real usage, feedback, and iteration.

### Engineering Matters

A useful product still needs reliable foundations, secure data handling, testing, observability, and maintainable code.

---

## 💳 About Membership

Membership and payments are intentionally **not a priority at the current stage**.

The first priority is proving that Kurika Meguri is genuinely useful.

```text
Build
  ↓
Deploy
  ↓
Get users
  ↓
Collect feedback
  ↓
Improve
  ↓
Validate demand
  ↓
Consider monetization
```

A deployed product with real users, feedback, iteration, and sound engineering is already a successful outcome.

---

## 🚧 Project Status

Kurika Meguri is currently in active development.

The project is starting from the fundamentals of Japanese learning, with **Hiragana** as the first learning module.

Development progress and upcoming work are tracked separately in [`TODO.md`](TODO.md).

---

<div align="center">

### Learn. Repeat. Remember.

**Kurika Meguri** — 繰り返して、身につける。

Built with 🦀 Rust

</div>