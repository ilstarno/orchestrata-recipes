# Traffic planning recipes

Use these recipes to review a traffic plan before changing DNS, a load balancer, or a deployment. They return proposals and review holds. They do not query infrastructure, make production changes, or represent a provider integration. Bring read-only snapshots from systems you administer, with opaque IDs and no secrets or customer identifiers.

| Recipe | Decision | Read the result |
| --- | --- | --- |
| [Regional DNS steering](../recipes/dns-regional-steering.xml) | Which allowed, healthy regions could receive new weighted DNS lookups? | Use the first qualifying tier. Leave the whole market on hold if no allowed region qualifies. Weights are not measured request distribution; cached answers may still be used. |
| [Backend capacity placement](../recipes/backend-capacity-placement.xml) | How many new requests could eligible backends accept after reserves? | Report new placement and the residual separately. A backend with stale or unknown health receives no proposed allocation. |
| [Canary, drain, and failback](../recipes/canary-drain-failback.xml) | Does the evidence support the next traffic step, waiting, a drain, or a return to service? | Require enough recent observations, current fallback headroom, and an agreed plan for in-flight work. Send incomplete evidence or conflicts to a person for review. |

The DNS and backend outputs are distinct. An edge can accept a portion of its known regional request load while remaining ineligible for *new DNS lookups*, for example when only one backend fault domain is healthy. An operator should check provider behavior, including failures and timeouts, before applying any recommendation. A weight of zero is not a universal isolation or draining control. There is no fallback to disallowed regions and no automatic DNS or load-balancer update.

The XML uses the Orchestrata Recipes v1 field format. List values use the row layouts documented in each field. The metadata loader checks fields and types, not relationships between rows; operational validation belongs to the planner. The worked comparison is separate from the XML. The canary recipe documents a review contract, not an automated deployment stage.

Provider references, not integration claims:

- [Amazon Route 53 weighted routing](https://docs.aws.amazon.com/en_en/Route53/latest/DeveloperGuide/routing-policy-weighted.html) and [DNS TTL and caching guidance](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/best-practices-dns.html).
- [AWS Application Load Balancer target-group behavior](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/edit-target-group-attributes.html) and [Kubernetes terminating endpoint behavior](https://kubernetes.io/docs/tutorials/services/pods-and-endpoint-termination-flow/).
