use assert_cmd::Command;
use mockito::Server;
use serde_json::json;

#[test]
fn run_help_text_mentions_required_and_optional_flags() {
    let mut cmd = Command::cargo_bin("synapse").expect("binary exists");
    cmd.args(["admin", "reconciliation", "run", "--help"]);
    let output = cmd.output().expect("help output");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("valid utf-8");

    assert!(stdout.contains("Required flags:"));
    assert!(stdout.contains("--account <ACCOUNT>"));
    assert!(stdout.contains("Optional flags:"));
    assert!(stdout.contains("--period-hours <HOURS>"));
}

#[tokio::test]
async fn run_command_prints_mock_server_summary() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/admin/reconciliation/run")
        .match_header("content-type", "application/json")
        .match_body(r#"{"account":"GA_TEST_ACCOUNT","period_hours":24}"#)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Reconciliation completed successfully",
                "report": {
                    "id": "3f1d8c31-5f1d-4fb8-93e0-112233445566",
                    "generated_at": "2026-06-27T06:10:12Z",
                    "period_start": "2026-06-26T06:10:12Z",
                    "period_end": "2026-06-27T06:10:12Z",
                    "total_db_transactions": 12,
                    "total_chain_payments": 11,
                    "missing_on_chain_count": 1,
                    "orphaned_payments_count": 0,
                    "amount_mismatches_count": 1,
                    "has_discrepancies": true
                }
            })
            .to_string()
        )
        .create_async()
        .await;

    let mut cmd = Command::cargo_bin("synapse").expect("binary exists");
    cmd.args([
        "--base-url",
        &server.url(),
        "admin",
        "reconciliation",
        "run",
        "--account",
        "GA_TEST_ACCOUNT",
    ]);

    let output = cmd.output().expect("command output");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("valid utf-8");

    assert!(stdout.contains("Reconciliation completed successfully"));
    assert!(stdout.contains("Report ID: 3f1d8c31-5f1d-4fb8-93e0-112233445566"));
    assert!(stdout.contains("Database transactions: 12"));
    assert!(stdout.contains("Has discrepancies: yes"));
}
