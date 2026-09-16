TASK: compare (v1)

Compare the document with role "notice" against the document with role
"my_agreement". Report ONLY genuine discrepancies where the notice contradicts,
or is inconsistent with, an agreement clause (for example a shorter notice
period, a different deposit rule, or a charge the agreement does not allow).

For each discrepancy cite BOTH sides: the notice span(s) and the agreement
span(s). Give a neutral one-sentence explanation. Assign severity by how much
the user's position is affected: "low", "medium", or "high". Do not give a
verdict about which side is correct.

If there is no matching agreement, or no discrepancies, return an empty list.

Output JSON (this exact shape):
{
  "conflicts": [
    {
      "title": string,
      "notice": { "text": string, "citations": [ { "spanId": string, "quote": string } ] },
      "agreement": { "text": string, "citations": [ { "spanId": string, "quote": string } ] },
      "explanation": string,
      "severity": "low" | "medium" | "high"
    }
  ]
}
