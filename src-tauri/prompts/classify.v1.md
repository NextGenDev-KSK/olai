TASK: classify (v1)

Identify what the primary document is, who the parties are, and give a one-line
plain summary. Cite spans for each.

Output JSON (this exact shape):
{
  "docType": { "text": string, "citations": [ { "spanId": string, "quote": string } ] },
  "parties": [
    { "role": string, "claim": { "text": string, "citations": [ { "spanId": string, "quote": string } ] } }
  ],
  "summary": { "text": string, "citations": [ { "spanId": string, "quote": string } ] }
}
