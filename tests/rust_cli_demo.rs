use serde_json::Value;
use std::process::Command;

fn run_orbit(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_orbit-dtl"))
        .args(args)
        .output()
        .expect("orbit-dtl binary should execute");

    assert!(
        output.status.success(),
        "orbit-dtl failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout should be utf8")
}

fn demo_json() -> Value {
    let stdout = run_orbit(&["demo", "--json"]);
    serde_json::from_str(&stdout).expect("demo json should parse")
}

#[test]
fn demo_texto_muestra_resumen_operativo() {
    let stdout = run_orbit(&["demo"]);

    assert!(stdout.contains("Orbit DTL demo executed"));
    assert!(stdout.contains("accounts: 3"));
    assert!(stdout.contains("tracked_balances: 3"));
    assert!(stdout.contains("vaults: 2"));
    assert!(stdout.contains("sessions: 1"));
    assert!(stdout.contains("events: 13"));
}

#[test]
fn demo_json_expone_cuentas_vaults_sesiones_y_eventos() {
    let report = demo_json();

    assert_eq!(report["accounts"].as_object().unwrap().len(), 3);
    assert_eq!(report["vaults"].as_object().unwrap().len(), 2);
    assert_eq!(report["sessions"].as_object().unwrap().len(), 1);
    assert_eq!(report["events"].as_array().unwrap().len(), 13);
}

#[test]
fn demo_json_cierra_la_sesion_con_intents_liquidados() {
    let report = demo_json();
    let sessions = report["sessions"].as_object().unwrap();
    let session = sessions.values().next().unwrap();
    let included = session["included_intents"].as_array().unwrap();

    assert_eq!(session["closed"], true);
    assert_eq!(included.len(), 2);
    assert_eq!(session["counterflow"]["1"], 10_500_000_000u64);
    assert_eq!(session["accounted_counterflow"]["1"], 19_759_259_259u64);
}

#[test]
fn eventos_de_demo_mantienen_flujo_de_liquidacion() {
    let report = demo_json();
    let events = report["events"].as_array().unwrap();
    let event_types = events
        .iter()
        .map(|event| event["type"].as_str().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(event_types[0], "asset_registered");
    assert_eq!(event_types[1], "asset_registered");
    assert_eq!(
        event_types
            .iter()
            .filter(|event_type| **event_type == "intent_queued")
            .count(),
        3
    );
    assert_eq!(
        event_types
            .iter()
            .filter(|event_type| **event_type == "intent_settled")
            .count(),
        2
    );
    assert!(event_types.contains(&"intent_cancelled"));
    assert!(event_types.contains(&"counterflow_recorded"));
}
