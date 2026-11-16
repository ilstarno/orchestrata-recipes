# Hiring operations recipes

These recipes cover the work around interviews. Start with requests a recruiter has already approved. The planner proposes times, panel coverage, rooms, and internal tasks. It does not score, rank, advance, or reject applicants. A scheduling hold goes to a person for review; it is not an applicant status.

| Recipe | What it checks | Worked scenario |
| --- | --- | --- |
| [Interview scheduling](../recipes/hiring-interview-scheduling.xml) | Confirmed availability, separate panel seats, medium and room fit, preparation and turnover, fixed bookings | Request 01 is available 14:00–15:00 UTC. It needs 30 minutes and two distinct panelists. Two eligible staff and a video room are available for the whole hour. With 14:00 and 14:05 offered and five minutes of preparation and turnover, the first feasible interview is 14:05–14:35 UTC. |
| [Interviewer recovery](../recipes/hiring-interviewer-recovery.xml) | An approved substitute, no overlap, minimal changes, frozen bookings | A panelist calls out. Keep the agreed time if another qualified panelist is free. If a frozen meeting has no feasible plan without changes, raise a review hold. |
| [Coordinator checkpoints](../recipes/hiring-coordinator-checkpoints.xml) | Available staff time, prerequisites, team-defined response targets | Leave a feedback reminder blocked until the interview has completed. Give the recruiter the blocking reason without reading the feedback or messaging anyone. |

The XML uses Orchestrata Recipes v1 field declarations. Structured rows described in the list fields are mapped by the caller; v1 checks field types, not the meaning of values inside each row. The recipes are specifications, not claims that a stock engine or integration executes them. The scheduling replay is separate from the XML; the coordinator checklist is not part of that replay.

Bring only opaque request IDs, confirmed free/busy windows, required meeting format, and roster roles. Do not supply candidate materials, calendar descriptions, absence reasons, or accommodation details. Confirm handling of exceptions, calendar writes, and messages with a recruiter. These examples do not send invitations, update an applicant system, or determine legal compliance.
