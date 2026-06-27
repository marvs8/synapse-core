# Synapse CLI

A command-line interface for the Synapse Core fiat gateway system.

## Installation

Install the CLI from the current directory:

```bash
cargo install --path .
```

This will build and install the `synapse` binary to your Cargo bin directory (usually `~/.cargo/bin/`).

## Configuration

The CLI resolves configuration in the following order:

1. **Command-line flags** (highest priority)
2. **Environment variables**
3. **Configuration file** (lowest priority)

### Environment Variables

- `SYNAPSE_API_URL` - Base URL for the Synapse API (default: `http://localhost:3000`)
- `SYNAPSE_AUTH_TOKEN` - Authentication token for API requests
- `SYNAPSE_OUTPUT_FORMAT` - Output format: `table` or `json` (default: `table`)

### Configuration File

Create `~/.synapse/config.json`:

```json
{
  "api_url": "http://localhost:3000",
  "output_format": "table"
}
```

## Output Modes

### Table Output (default)

```bash
synapse transactions list
```

Output:
```
ID                                   Status      Amount    Asset   Created At
12345678-1234-1234-1234-123456789012 completed   100.00    USD     2026-01-15T10:30:00Z
87654321-4321-4321-4321-210987654321 pending     50.00     EUR     2026-01-15T11:45:00Z
```

### JSON Output

```bash
synapse transactions list --json
```

Output:
```json
[
  {
    "id": "12345678-1234-1234-1234-123456789012",
    "status": "completed",
    "amount": 100.00,
    "asset": "USD",
    "created_at": "2026-01-15T10:30:00Z"
  }
]
```

## Exit Codes

The CLI uses the following exit codes:

- **0** - Success
- **1** - General error
- **2** - Authentication failure (invalid credentials, expired token)
- **3** - Resource not found (404)

Examples:

```bash
synapse health
echo $?  # Outputs: 0

synapse transactions get invalid-id
echo $?  # Outputs: 3 (not found)

synapse transactions get --token invalid
echo $?  # Outputs: 2 (auth failure)
```

## Commands

### Health Check

Verify CLI connectivity and basic health:

```bash
synapse health
```

Expected output:
```
✓ Health check passed
```

### Transactions

#### List Transactions

List all transactions:

```bash
synapse transactions list
```

Filter by status:

```bash
synapse transactions list --status completed
synapse transactions list --status pending
```

Output:
```
ID                                   Status      Amount    Asset   Created At
12345678-1234-1234-1234-123456789012 completed   100.00    USD     2026-01-15T10:30:00Z
87654321-4321-4321-4321-210987654321 pending     50.00     EUR     2026-01-15T11:45:00Z
```

#### Search Transactions

Search transactions by filters:

```bash
synapse transactions search --asset USD
synapse transactions search --status completed --asset EUR
```

Output:
```
ID                                   Status      Amount    Asset   Created At
12345678-1234-1234-1234-123456789012 completed   100.00    USD     2026-01-15T10:30:00Z
```

#### Get Transaction Details

Get a specific transaction:

```bash
synapse transactions get 12345678-1234-1234-1234-123456789012
```

Output:
```
ID:          12345678-1234-1234-1234-123456789012
Status:      completed
Amount:      100.00
Asset:       USD
Created At:  2026-01-15T10:30:00Z
Updated At:  2026-01-15T10:35:00Z
```

### Settlements

#### List Settlements

List all settlements:

```bash
synapse settlements list
```

Output:
```
ID                                   Date       Total Amount Asset   Status
aaaa1111-bbbb-cccc-dddd-eeeeffffffff 2026-01-15 5000.00      USD     completed
bbbb2222-aaaa-cccc-dddd-eeeeffffffff 2026-01-16 3500.00      EUR     pending
```

#### Get Settlement Details

Get a specific settlement:

```bash
synapse settlements get aaaa1111-bbbb-cccc-dddd-eeeeffffffff
```

Output:
```
ID:            aaaa1111-bbbb-cccc-dddd-eeeeffffffff
Date:          2026-01-15
Total Amount:  5000.00
Asset:         USD
Status:        completed
Transactions:  42
```

## Shell Completions

Generate shell completions to enable tab completion in your shell.

### Bash

```bash
synapse completions bash > ~/.bash_completions.d/synapse
source ~/.bash_completions.d/synapse
```

### Zsh

```bash
synapse completions zsh > ~/.zsh/completions/_synapse
```

### Fish

```bash
synapse completions fish > ~/.config/fish/completions/synapse.fish
```

After generating completions, restart your shell or run `hash -r` (bash/zsh) to activate them.

## Testing with Mock Server

The CLI can be tested against the mock server included in the integration test harness.

### Start the Mock Server

From the repository root:

```bash
cargo run --bin synapse-mock-server
```

The mock server will start on `http://localhost:3000`.

### Point CLI to Mock Server

```bash
export SYNAPSE_API_URL=http://localhost:3000
synapse health
```

Or use the flag:

```bash
synapse --api-url http://localhost:3000 health
```

## Troubleshooting

### Authentication Errors (exit code 2)

- Verify `SYNAPSE_AUTH_TOKEN` is set correctly
- Check that the token has not expired
- Ensure the API endpoint is correct

```bash
export SYNAPSE_AUTH_TOKEN=your-valid-token
synapse health
```

### Resource Not Found (exit code 3)

- Verify the resource ID is correct
- Check that the resource exists on the server

```bash
synapse transactions get 12345678-1234-1234-1234-123456789012
```

### General Errors (exit code 1)

- Check network connectivity to the API server
- Verify environment variables are set correctly
- Run with verbose logging (if supported)

## Development

Build the CLI in development mode:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Format code:

```bash
cargo fmt
```

Check for issues:

```bash
cargo clippy
```

## License

Part of the Synapse Core project.
