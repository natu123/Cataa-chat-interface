# Cataa — Multi-party CLI Chat Interface

Cataa is a Rust CLI chat shell where humans and AI agents talk together in the same terminal.

> Cataa is also a reference design for future chat UI layers in the [Yunin](https://github.com/natu123/yunin-ai-workspace) project.

---

## Conversation Format

```
Gles : Hello, Lavi!
       How are you today?
Lavi : Hello.
Loa  : Both of you seem to be doing well.
```

- Speaker prefix uses `:` — keeps `>` free for quoting
- Multi-line messages are indented to align with the speaker name
- Each speaker is displayed in a distinct color

---

## How Loa is Connected

Cataa launches the `claude` command as a **subprocess** and controls its stdin/stdout directly.

```
Cataa (Rust)
  ├── [Gles input]  → write to stdin
  ├── [Lavi output] → write to stdin
  └── read stdout   → display as [Loa]
```

This preserves the full conversation context — Loa sees both Gles and Lavi as part of the same ongoing conversation.

---

## Tech Stack

- **Language**: Rust
- **Terminal colors**: colored crate
- **subprocess control**: std::process

---

## Roadmap

- [ ] Step 1: 3-party chat shell (colored speaker display)
- [ ] Step 2: Lavi integration (local process)
- [ ] Step 3: Loa subprocess communication
- [ ] Step 4: Yunin UI reference implementation

---

## Related

- [lavi-continual-learning](https://github.com/natu123/lavi-continual-learning) — the AI that Cataa talks to
- [yunin-ai-workspace](https://github.com/natu123/yunin-ai-workspace) — future integration target

---

## Author

**Gles** (増田 賢治) — [@____natu______](https://x.com/____natu______)  
Designed by Gles. Implemented by Loa (Claude Code).
