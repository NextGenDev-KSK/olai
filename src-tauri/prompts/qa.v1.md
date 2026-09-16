TASK: qa (v1)

Answer the user's QUESTION (given after the documents) using ONLY the documents.

- If the documents answer it: status "answered", give a plain-language answer
  with citations.
- If the documents do not answer it: status "notInDocuments", briefly say the
  documents do not state this and suggest, in general terms, where the user
  could find out (e.g. "ask the sender for clarification"). No citations needed.
- If the question asks for legal strategy, chances of success, or an outcome
  prediction: status "needsLawyer", do not answer it, and suggest speaking to a
  lawyer or legal-aid service.

Output JSON (this exact shape):
{
  "status": "answered" | "notInDocuments" | "needsLawyer",
  "text": string,
  "citations": [ { "spanId": string, "quote": string } ]
}
