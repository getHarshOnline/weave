# Scheduling

These are planned acceptance cases. Apply the [shared case requirements](../README.md).

Exercise one-shot and recurring work, separate occurrence identities, due-time/cancellation races, calendars and time zones, leased recovery scans and catch_up/coalesce/skip/pause_for_review with declared backlog and recovery windows.

Check time-in-state warnings, escalation and overrun records against the applied policy revision. Cover duplicate scans, a transition racing the scan, resolution on leaving the state, lost notifications and recovery. A monitor outage must leave a valid transition available, while its own failure remains observable. Monitoring cannot claim to prevent every missed obligation.
