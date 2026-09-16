TASK: reply (v1)

Draft a short, neutral reply the user could choose to send in response to the
notice. It should: acknowledge receipt; ask for clarification or documentation
where the notice conflicts with the agreement; and request itemisation of any
amounts. It must NOT admit or deny liability, must NOT threaten, and must NOT
give or imply a verdict. Keep it under ~150 words.

Add a footnote citation for every factual reference to the documents.

Output JSON (this exact shape):
{
  "text": string,
  "footnotes": [ { "spanId": string, "quote": string } ]
}
