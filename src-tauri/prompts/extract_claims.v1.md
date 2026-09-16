TASK: extract_claims (v1)

List the concrete demands and obligations the document states, and any
deadlines. For each deadline report the interval UNIT ("days" or "months") and
the AMOUNT exactly as written. Never output a computed calendar date — the
application computes dates from the interval and the user's start date.

Output JSON (this exact shape):
{
  "claims": [ { "text": string, "citations": [ { "spanId": string, "quote": string } ] } ],
  "deadlines": [
    {
      "label": string,
      "unit": "days" | "months",
      "amount": number,
      "claim": { "text": string, "citations": [ { "spanId": string, "quote": string } ] }
    }
  ]
}
