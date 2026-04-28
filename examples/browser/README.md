# browser use

```bash
uv init
uv add browser-use
uv add python-dotenv
uvx ruff format
```

```bash
echo '.env' > .gitignore
echo 'ANTHROPIC_API_KEY="..."' > .env
```

```bash
source .venv/bin/activate
python hello.py
```

- model: `claude-sonnet-4-0`
- input tokens: 56,982
- output tokens: 1,967
- $ 0.15

```bash
INFO     [service] 🖼️  Auto-configured LLM screenshot size for Claude Sonnet: 1400x850
INFO     [service] Using anonymized telemetry, see https://docs.browser-use.com/development/monitoring/telemetry.
INFO     [Agent] 🎯 Task: Find the number 1 post on Show HN
INFO     [Agent] Starting a browser-use agent with version 0.12.6, with provider=anthropic and model=claude-sonnet-4-0
INFO     [Agent]

INFO     [Agent] 📍 Step 1:
INFO     [Agent]   ❔ Eval: No previous action to evaluate - this is the first step.
INFO     [Agent]   🧠 Memory: Starting task to find the #1 post on Show HN. Currently on blank page, need to navigate to Hacker News first.
INFO     [Agent]   🎯 Next goal: Navigate to Hacker News website to access the Show HN section.
INFO     [Agent]   ▶️   navigate: url: https://news.ycombinator.com, new_tab: False
INFO     [tools] 🔗 Navigated to https://news.ycombinator.com
INFO     [Agent]

INFO     [Agent] 📍 Step 2:
INFO     [prompts] 🔄 Resizing screenshot from 1936x1038 to 1400x850 for LLM
INFO     [Agent]   👍 Eval: Successfully navigated to Hacker News main page. Verdict: Success
INFO     [Agent]   🧠 Memory: Currently on Hacker News main page. Can see several Show HN posts mixed with regular posts, but need to navigate to the dedicated Show HN section to find the #1 Show HN post specifically.
INFO     [Agent]   🎯 Next goal: Click on the "show" link to navigate to the Show HN section to find the #1 Show HN post.
INFO     [Agent]   ▶️   click: index: 52, coordinate_x: None, coordinate_y: None
INFO     [tools] 🖱️ Clicked a "show"
INFO     [Agent]

INFO     [Agent] 📍 Step 3:
INFO     [prompts] 🔄 Resizing screenshot from 1936x1038 to 1400x850 for LLM
INFO     [Agent]   👍 Eval: Successfully clicked on the "show" link and navigated to the Show HN section. Verdict: Success
INFO     [Agent]   🧠 Memory: Successfully navigated to the Show HN section and can see the ranked list of Show HN posts. Found the #1 post which is "Show HN: AI memory with biological decay (52% recall)" by SachitRafa with 48 points, posted 3 hours ago with 21 comments.
INFO     [Agent]   🎯 Next goal: Task completed - found the #1 Show HN post. Ready to report the findings to the user.
INFO     [Agent]   ▶️   done: text: I found the #1 post on Show HN:

**Title:** Show HN: AI memory with biological decay (52% recall)
**Author:** SachitRafa
**Points:** 48 points
**Poste..., success: True, files_to_display: []
INFO     [Agent]
📄  Final Result:
I found the #1 post on Show HN:

**Title:** Show HN: AI memory with biological decay (52% recall)
**Author:** SachitRafa
**Points:** 48 points
**Posted:** 3 hours ago
**Comments:** 21 comments
**Link:** github.com/sachitrafa

This is currently the top-ranked post in the Show HN section of Hacker News.


INFO     [Agent] ✅ Task completed successfully
INFO     [BrowserSession] 📢 on_BrowserStopEvent - Calling reset() (force=True, keep_alive=None)
INFO     [BrowserSession] [SessionManager] Cleared all owned data (targets, sessions, mappings)
INFO     [BrowserSession] ✅ Browser session reset complete
INFO     [BrowserSession] ✅ Browser session reset complete
```

