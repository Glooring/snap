# 🚀 Pro Tip: Refactoring "God Objects" (5k+ lines) with Zero Hallucinations

**Subject:** How to split massive Monoliths using Agentic Planning & Scripting (The "Architect-Executor" Pattern)

Hey everyone! 👋

I recently successfully refactored a massive **7,000-line TypeScript file** (`src/index.ts`) into a clean, modular architecture (5 separate files) without a single broken line of logic.

We all know the pain of asking an LLM to "refactor this huge file":
1.  ❌ **Truncation:** The output cuts off because of token limits.
2.  ❌ **Hallucination:** It rewrites logic slightly, introducing subtle bugs.
3.  ❌ **Lazy Coding:** It summarizes parts you wanted to keep (`// ... rest of code`).

**💡 The Solution: The "Architect & Script" Workflow**

The secret isn't to ask the AI to *write the code*. The secret is to use a 2-Phase approach where the AI first acts as a **Thoughtful Architect** (analyzing logic without mapping lines), and then as a **Precise Executor** (mapping lines and writing a script).

Here is the exact workflow that works perfectly in Google Antigravity (or any Agentic IDE):

---

### Phase 1: The Architect (Deep Analysis)

In this phase, you ask the AI to understand the *semantics* of your code, not the syntax. It shouldn't map line numbers yet.

**Prompt:**
> "Analyze `src/index.ts` and the surrounding codebase to understand the context. It is too monolithic.
> 1. **Analyze Logic:** Identify the major logical components (e.g., Global State, UI, Core Loop, Config).
> 2. **Propose Hierarchy:** Suggest a new folder/file structure based on your analysis.
> 3. **Create Plan:** Create a file named `REFACTOR_PROMPT.md` in the root. In this file, write a detailed Plan for a future AI Agent.
>    - The plan should include your proposed hierarchy as a **strong recommendation**, but instruct the Agent to be flexible if needed during detailed mapping.
>    - The plan MUST ask the Agent to create a **Node.js script** (`refactor_splitter.js`) to programmatically extract the lines to ensure 100% integrity."

*Result:* You get a `REFACTOR_PROMPT.md` that contains a smart, context-aware architectural plan.

---

### Phase 2: The Executor (Agent Mode)

Now, you can create a new chat to have fresh context, switch to Planning mode, and put this prompt.

**The Trigger Prompt:**
> "Read and execute the plan from `REFACTOR_PROMPT.md`. Start by mapping the specific line ranges from `src/index.ts` to the proposed modules."

**The Result (What the Agent does):**
The Agent now reads the huge file specifically to map line numbers (1-50 -> Config, 51-200 -> State, etc.). It adapts the Architect's plan if needed.

*Agent Output Log:*
```text
Refactoring index.ts
Mapped src/index.ts to target files (Constants, GameState, UI, etc.)

Implementation Plan:
[Task 1] Mapping content of src/index.ts (lines 1-2000)
[Task 2] Mapping content of src/index.ts (lines 2001-4000)
...
[Task 6] Creating refactor_splitter.js script
```

**Why this works:**
1.  **Context-Aware Architecture:** Phase 1 ensures the split makes *logical* sense, not just syntactic sense.
2.  **Zero Hallucinations:** Phase 2 uses a script (`fs.readFile` + `slice`) to move code. Logic is preserved perfectly.
3.  **Infinite Scale:** You can refactor a 50k line file this way. The AI isn't outputting code; it's outputting *instructions* for a script.

I used this to split my entire engine core. `tsc` compiled on the very first try after the script ran.

If you have a "God Class" you're scared to touch, try the **Architect -> Script** pattern! 🛠️
