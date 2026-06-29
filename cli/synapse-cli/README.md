# synapse-cli

`synapse-cli` is a small CLI for working with Synapse admin reconciliation endpoints.

## Commands

The reconciliation tree is:

- `synapse admin reconciliation reports`
- `synapse admin reconciliation report <REPORT_ID>`
- `synapse admin reconciliation run --account <ACCOUNT> [--period-hours <HOURS>]`

The settlement tree is:

- `synapse admin settlements update-status <SETTLEMENT_ID> --status <STATUS> [--reason <REASON>] [--new-total <TOTAL>] [--actor <ACTOR>] [--json]`

The help text spells out required and optional flags for each subcommand. For example:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml -- admin reconciliation run --help
```

## Example

In one terminal, start the mock API:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml --bin mock-server
```

Then run a reconciliation against it and print the resulting summary:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml -- `
  --base-url http://127.0.0.1:4010 `
  admin reconciliation run `
  --account GA_TEST_ACCOUNT `
  --period-hours 24
```

Sample output:

```text
Reconciliation completed successfully

Report ID: 3f1d8c31-5f1d-4fb8-93e0-112233445566
Generated: 2026-06-27T06:10:12Z
Period: 2026-06-26T06:10:12Z to 2026-06-27T06:10:12Z

Summary:
  Database transactions: 12
  Chain payments: 11
  Missing on chain: 1
  Orphaned payments: 0
  Amount mismatches: 1
  Has discrepancies: yes
```

## Settlement Example

In one terminal, start the mock API:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml --bin mock-server
```

Then update a settlement status against it and print the resulting settlement:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml -- `
  --base-url http://127.0.0.1:4010 `
  admin settlements update-status `
  8f9b0f0c-9a89-4d1f-9d7d-0c7d7d0d9a11 `
  --status adjusted `
  --reason "Audit correction" `
  --new-total 125.0000000
```

Sample output:

```text
Settlement updated successfully

Settlement ID: 8f9b0f0c-9a89-4d1f-9d7d-0c7d7d0d9a11
Asset code: USDC
Status: adjusted
Total amount: 125.0000000
Tx count: 8
Period: 2026-06-26T00:00:00Z to 2026-06-27T00:00:00Z
Dispute reason: Audit correction
Original total amount: 130.0000000
Reviewed by: admin
Reviewed at: 2026-06-27T09:15:00Z
Created at: 2026-06-27T09:00:00Z
Updated at: 2026-06-27T09:15:00Z
```
