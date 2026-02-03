# vibe-review

Local CLI-based Rust code review bot for the Vibe project.

## Usage

```bash
# Review last commit
vibe-review

# Review specific commit
vibe-review --commit abc1234

# Review branch comparison (PR simulation)
vibe-review --base main --head feature/my-branch

# Review staged changes (pre-commit check)
vibe-review --staged

# Specify output file
vibe-review --output review.md

# Preview prompt without API call
vibe-review --dry-run
```

## Environment

Set the Claude API key before running:

```bash
export ANTHROPIC_API_KEY="..."
```

