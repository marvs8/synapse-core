# Synapse CLI

Command-line interface for managing transactions and settlements in Synapse.

## Installation

Build the CLI:

```bash
cargo build --release
```

The binary will be available at `target/release/synapse`.

## Configuration

Set the Synapse API URL and optional API key via environment variables:

```bash
export SYNAPSE_URL=http://localhost:3000
export SYNAPSE_API_KEY=your-api-key
```

Or pass them as command-line flags:

```bash
synapse --url http://localhost:3000 --api-key your-api-key [COMMAND]
```

## Commands

### Transactions

#### Export Transactions

Export transactions with optional filters to CSV or JSON format.

```bash
synapse transactions export [OPTIONS]
```

**Options:**
- `--format <FORMAT>`: Export format - `csv` (default) or `json`
- `--from <FROM>`: Start date filter (YYYY-MM-DD)
- `--to <TO>`: End date filter (YYYY-MM-DD)
- `--status <STATUS>`: Filter by transaction status (e.g., pending, completed)
- `--asset-code <ASSET_CODE>`: Filter by asset code (e.g., USD, EUR)
- `--output <OUTPUT>`: Save to file instead of stdout

**Examples:**

Export all transactions as CSV to stdout:
```bash
synapse transactions export
```

Export pending USD transactions as JSON:
```bash
synapse transactions export --format json --status pending --asset-code USD
```

Export transactions from last 30 days to a file:
```bash
synapse transactions export --from 2024-01-01 --to 2024-01-31 --output transactions.csv
```

### Settlements

#### List Settlements

List settlements with cursor-based pagination.

```bash
synapse settlements list [OPTIONS]
```

**Options:**
- `--cursor <CURSOR>`: Start from a specific cursor
- `--limit <LIMIT>`: Number of results per page (1-100, default: 10)
- `--direction <DIRECTION>`: Pagination direction - `forward` (default) or `backward`
- `--format <FORMAT>`: Output format - `table` (default) or `json`

**Examples:**

List first 10 settlements:
```bash
synapse settlements list
```

List 50 settlements in JSON format:
```bash
synapse settlements list --limit 50 --format json
```

Navigate with cursor:
```bash
synapse settlements list --cursor <cursor-from-previous-response> --limit 25
```

#### Get Settlement

Get details of a specific settlement.

```bash
synapse settlements get <SETTLEMENT_ID> [OPTIONS]
```

**Arguments:**
- `SETTLEMENT_ID`: The settlement UUID

**Options:**
- `--format <FORMAT>`: Output format - `table` (default) or `json`

**Examples:**

Get settlement details in table format:
```bash
synapse settlements get 550e8400-e29b-41d4-a716-446655440000
```

Get settlement details in JSON format:
```bash
synapse settlements get 550e8400-e29b-41d4-a716-446655440000 --format json
```

## Output Formats

### Table Format (default)
Human-readable output with columns for lists and key-value pairs for objects.

### JSON Format
Full JSON output with all fields, useful for scripting and integration.

## Testing

Run tests:

```bash
cargo test
```

Tests requiring external services are marked with `#[ignore]` and can be run with:

```bash
cargo test -- --ignored
```

## Troubleshooting

### Connection Refused
Ensure the Synapse API server is running and the `--url` or `SYNAPSE_URL` environment variable is correctly set.

### Invalid UUID
Settlement IDs must be valid UUIDs (format: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`).

### Empty Results
When exporting transactions or listing settlements returns no results:
- Verify filter parameters are correct
- Check date ranges (use YYYY-MM-DD format)
- Confirm the asset code or status value exists
