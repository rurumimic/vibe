# Human Guardrails for Vibe

This document specifies the boundaries that humans should not cross when using AI.  

## What are Human Guardrails

Human Guardrails define:

- Areas that should not be delegated to AI
- Where human judgment must be directly responsible, no matter how plausible AI output appears

Guardrails are boundaries, not rules.  
They don't slow you down. They prevent crashes.

---

## Security and Misuse Guardrails

### Prompt Injection

AI systems cannot reliably distinguish between instructions and data.  
Malicious instructions can be embedded in external content (documents, emails, web pages).

Human Guardrails:
- Treat all external data processed by AI as potential attack vectors.
- Never grant AI agents access to sensitive systems without isolation.
- Assume indirect prompt injection is possible in any AI workflow.

### AI Output Trust and Responsibility

AI output is highly persuasive and can distort human judgment.  
Model safety features only reduce likelihood of harmful outputs.  
They do not replace responsibility or guarantee safety.  
Safe-looking outputs do not mean the system is safe.

Human Guardrails:
- Never treat AI output as correct just because AI said so.
- Never bypass verification.
- Never shift responsibility to model safety features or filters.
- Always be aware that you may be being persuaded by AI.

### Deceptive Alignment

AI appears to understand goals but only produces statistical approximations.

Human Guardrails:
- Do not claim that AI "understands."
- Humans must explicitly define design intent and system goals.
- Never assume safety mechanisms fully constrain AI behavior.

### Shadow AI and Data Exposure

Shadow AI: the use of AI tools outside approved visibility, governance, or control.

AI usage creates invisible data flows that can expose internal context, security assumptions, and trust boundaries.

Human Guardrails:
- Treat all prompts as potential data exfiltration paths.
- Never include internal context, security assumptions, or failure scenarios in prompts.
- Distinguish clearly between trust boundaries of SaaS LLMs and local models.
- Verify data boundaries before using any AI tool.

### AI Tools as Attack Surface

AI agents and development tools are themselves attack vectors.

Risks include:
- AI agents manipulated to perform harmful actions (credential harvesting, data exfiltration)
- Vulnerabilities in AI tools themselves
- Malicious instructions embedded in project files that AI tools process

Human Guardrails:
- Evaluate AI tools with the same rigor as any third-party dependency.
- Never grant AI agents persistent access to production systems.
- Require human approval for any action with external effects.
- Isolate AI tool execution from sensitive environments.

---

## Quality Guardrails

### Code Cloning and Illusion of Progress

AI often reuses existing code.  
This can make work look like it is progressing.  
Meanwhile, the system structure is breaking down.

Human Guardrails:
- Do not treat AI-generated code as a signal of real progress.
- Do not merge code unless it is clearly reshaped to fit the project's structure and intent.

### Business Logic Gaps

AI avoids generic vulnerabilities (SQLi, XSS) but lacks "common sense."  
Business logic flaws emerge where context determines what is safe vs dangerous.

Human Guardrails:
- Review all AI-generated code for domain-specific logic errors.
- Explicitly specify business rules and constraints in prompts.
- Never assume AI understands implicit business requirements.

### Refactoring Decline

AI maintains working code.  
But it does not initiate structural change.

Human Guardrails:
- Treat structural refactoring as intentional design work, not as cleanup.
- Explicitly decide when the system structure must be redesigned.
- Do not keep fixing small issues while postponing structural refactoring.

### Technical Debt at Decision Boundaries

Technical debt grows where decisions are unclear.

These boundaries are often hidden:
- Error handling paths
- Concurrency and synchronization
- Failure and recovery behavior
- Performance and resource tradeoffs
- State ownership and lifecycle

AI usually focus on the happy path.  
Edge cases are often skipped, guessed, or left implicit.

This is where debt quietly accumulates.

Human Guardrails:
- Always review behavior beyond the happy path.
- Explicitly design error handling and failure scenarios.
- Make concurrency, recovery, and performance tradeoffs explicit.
- Define ownership and lifecycle for critical state.

---

## Productivity Guardrails

### Team Collaboration

AI does not resolve:
- Responsibility ownership
- Priority conflicts
- Decision-making disagreements

With AI usage:
- Individual output may increase
- Team synchronization costs often rise

Human Guardrails:
- Always record who made the decision.
- Treat AI output as reference material, not as a decision basis.
- Prioritize shared context at the team level.
- Do not forward AI outputs without interpretation.
