TASK: actions (v1)

Produce a short, practical checklist of neutral next steps, grouped into:
- "protect": preserve the user's position (keep copies, do not ignore it)
- "gather": documents/evidence to collect
- "respond": how to acknowledge or reply factually
- "getHelp": seek free legal aid or a lawyer

Give only neutral, procedural steps. No legal strategy, no predictions, no
verdicts. Cite spans where a step refers to something specific in the documents.

Output JSON (this exact shape):
{
  "actions": [
    {
      "group": "protect" | "gather" | "respond" | "getHelp",
      "text": string,
      "citations": [ { "spanId": string, "quote": string } ]
    }
  ]
}
