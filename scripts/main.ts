#!/usr/bin/env bun
import { cac } from "cac";
import { runPostJobFeedbackCLI } from "./gh-actions/ci-feedback.ts";

const cli = cac("scripts");

cli
  .command(
    "ci-feedback-post-job",
    "Post CI failure feedback from within a job (post-job approach)",
  )
  .action(() => {
    runPostJobFeedbackCLI();
  });

cli.command("", "Show help").action(() => {
  const helpText = `
# Dinosaur Game Scripts

This script provides helper utilities for CI/CD automation.

## Usage

\`\`\`bash
bun run main.ts <command>
\`\`\`

## Commands

### \`ci-feedback-post-job\`

Automatically posts CI failure details to your PR and asks Copilot to help fix the issues.

**How it works:**
When your CI fails, this command collects the error logs and posts a helpful comment on the PR
that mentions @copilot, so Copilot can analyze the failures and suggest fixes.

**🔑 One-time Setup: Create a Personal Access Token**

Since Copilot only responds to comments from real users (not bots), you need to create a
Personal Access Token (PAT) so the comment appears to come from your account.

**Quick Setup (Fine-grained Token - Recommended):**

1. Open GitHub: Settings → Developer settings → Personal access tokens → Fine-grained tokens
2. Click "Generate new token"
3. Name it something like "CI Copilot Helper"
4. Choose an expiration (or no expiration)
5. Under "Repository access", select this repository
6. Set these permissions:
   - Pull requests: Read and write ✏️
   - Actions: Read 👁️
   - Contents: Read 👁️
7. Click "Generate token" and copy it
8. In your repo: Settings → Secrets → Actions → New secret
9. Name: \`COPILOT_INVOKER_TOKEN\`, Value: paste your token

That's it! Now when CI fails on a PR, Copilot will automatically be asked to help.

---
Run \`bun run main.ts --help\` for CLI details.
`;
  console.log(helpText);
});

cli.help();
cli.parse();
