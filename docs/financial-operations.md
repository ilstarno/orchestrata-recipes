# Financial operations recipes

| Recipe | Operational decision | Stops at |
| --- | --- | --- |
| [Payment operations review routing](../recipes/payment-operations-review-routing.xml) | Match existing work to a permitted reviewer before its due time | No payment disposition, fraud inference, account changes or money movement |
| [Settlement reconciliation backlog](../recipes/settlement-reconciliation-backlog.xml) | Arrange exception review and independent checks before close | No factual match, journal, payout, or claim of a reconciled balance |
| [Claims operations work dispatch](../recipes/claims-operations-work-dispatch.xml) | Assign already-authorized administrative and inspection work to verified capacity | No policy eligibility, coverage, severity, liability, fraud classification or settlement |
| [Adjuster appointment recovery](../recipes/adjuster-appointment-recovery.xml) | Find authorized replacement coverage or propose a confirmed alternate window | No customer contact, calendar edits, visit cancellation, or claims decisions |

Use opaque task and staff references, aggregate or measured capacity, externally set due times and permissions, coarse zones, and verified source revisions. Drop payment amounts, credentials, records about individual customers, and the reason a staff member is unavailable. Missing evidence becomes a worklist hold, not a disposition. A person checks the proposal against the authoritative system before any action.

The XML follows the Orchestrata Recipes v1 field metadata format. It describes the inputs/outputs for review; it does not execute a workflow or connect to a provider. A separate replay can illustrate the work allocation, not certify financial, claims, privacy, or regulatory controls.

[Stripe's payout reconciliation report documentation](https://docs.stripe.com/reports/payout-reconciliation) is one example of an authoritative processor-side export. Such an export is separate from the scheduling proposal and does not independently prove agreement with a bank's records.
