# Retail staffing what-if: one absence, two stores

All data is fictional. At 14:00, the North and South stores each need two workers for concurrent shifts. The baseline uses the manager's prequalified choices. The what-if removes Ben from the two slots he could cover; the four shifts, hard constraints, and preference weights do not change.

| Shift | Baseline | Ben unavailable | Reason |
| --- | --- | --- | --- |
| North keyholder | Ana | Ana | Ana is pinned by the singleton domain in both problems. |
| North floor | Ben | Clara | After Ben is removed, Clara is the only eligible candidate. |
| South register | Clara | Elena | Clara now covers the simultaneous North shift and cannot be double-booked. |
| South stock | Diego | Diego | Diego remains eligible and preferred. |

Both plans cover all four shifts and have a hard score of zero. The adjusted soft score is -8: it misses the North floor preference for Ben (5) and the South register preference for Clara (3). Those preferences are recorded but cannot make an unavailable or double-booked assignment valid. The exact solver checks 12 baseline candidates and 4 adjusted candidates.

With the Orchestrata CLI installed, run from the repository root:

```sh
orchestrata-cli recipes/examples/retail-staffing-what-if/baseline/problem.json
orchestrata-cli recipes/examples/retail-staffing-what-if/adjusted/problem.json
```

Compare with the [expected baseline](baseline/expected.json) and [expected what-if plan](adjusted/expected.json). This is an assignment example, not a staffing forecast, labor compliance assessment, or service-level guarantee.
