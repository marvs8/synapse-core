# Synapse Core CLI

The Synapse Core CLI provides command-line tools for managing settlements, transactions, and database operations.

## Commands

### Transactions Search

Search for transactions using flexible filters and cursor-based pagination.

**Syntax:**
```bash
synapse-core tx search [OPTIONS]
```

**Filters (all optional):**
- `--status <STATUS>` – Filter by transaction status: `pending`, `processing`, `completed`, `failed`
- `--asset-code <CODE>` – Filter by asset code (e.g., `USD`, `EUR`)
- `--min-amount <AMOUNT>` – Inclusive minimum amount (decimal string, e.g., `100.00`)
- `--max-amount <AMOUNT>` – Inclusive maximum amount (decimal string, e.g., `500.00`)
- `--from <DATE>` – Inclusive start date (ISO 8601 format, e.g., `2024-01-01T00:00:00Z`)
- `--to <DATE>` – Exclusive end date (ISO 8601 format)
- `--stellar-account <ACCOUNT>` – Filter by Stellar account
- `--cursor <CURSOR>` – Pagination cursor from previous response
- `--limit <N>` – Results per page (default: 25, max: 100)
- `--format <FORMAT>` – Output format: `table` (default) or `json`

**Examples:**

Search all completed transactions:
```bash
synapse-core tx search --status completed
```

Search USD transactions with amount between 100 and 500:
```bash
synapse-core tx search --asset-code USD --min-amount 100.00 --max-amount 500.00
```

Search transactions in a date range:
```bash
synapse-core tx search --from 2024-01-01T00:00:00Z --to 2024-01-31T23:59:59Z
```

Output results as JSON:
```bash
synapse-core tx search --status pending --format json
```

**Table Output:**
```
ID                                   STATUS       ASSET        AMOUNT         
550e8400-e29b-41d4-a716-446655440000 pending      USD          100.00         
a1b2c3d4-e5f6-47g8-h9i0-j1k2l3m4n5o6 completed    USD          250.50         
x7y8z9a0-b1c2-43d4-e5f6-g7h8i9j0k1l2 processing   EUR          500.00         

✓ 3 results (total: 150)
  Use --cursor eyJpZCI6ICJ4N3k4ejlhMC1iMWMyLTQzZDQtZTVmNi1nN2g4aTlqMGsxbDIiLCAiY3JlYXRlZF9hdCI6ICIyMDI0LTAxLTE1VDEwOjAwOjAwWiJ9 for next page
```

**JSON Output:**
```json
{
  "total": 150,
  "results": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "stellar_account": "GABC1234567890123456789012345678901234567890123456789012",
      "amount": "100.00",
      "asset_code": "USD",
      "status": "pending",
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:00:00Z",
      "anchor_transaction_id": null,
      "callback_type": null,
      "callback_status": null,
      "settlement_id": null,
      "memo": null,
      "memo_type": null,
      "metadata": null
    }
  ],
  "next_cursor": "eyJpZCI6ICI1NTBlODQwMC1lMjliLTQxZDQtYTcxNi00NDY2NTU0NDAwMDAiLCAiY3JlYXRlZF9hdCI6ICIyMDI0LTAxLTE1VDEwOjAwOjAwWiJ9"
}
```

### Settlements List

List all settlements with cursor-based pagination.

**Syntax:**
```bash
synapse-core settlements list [--format <FORMAT>]
```

**Example:**
```bash
synapse-core settlements list --format table
synapse-core settlements list --format json
```

### Settlements Get

Retrieve a specific settlement by ID.

**Syntax:**
```bash
synapse-core settlements get <SETTLEMENT_ID> [--format <FORMAT>]
```

**Example:**
```bash
synapse-core settlements get 550e8400-e29b-41d4-a716-446655440000
```

Returns HTTP 404 if settlement not found.

## Configuration

The CLI reads configuration from environment variables:
- `SYNAPSE_API_URL` – API base URL (default: `http://localhost:3000`)
- `SYNAPSE_API_KEY` – API key for authentication (default: `dev-key`)

Example:
```bash
SYNAPSE_API_URL=https://api.synapse.example.com \
SYNAPSE_API_KEY=sk_live_xyz123 \
synapse-core tx search --status completed
```
