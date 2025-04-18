use std::fs;
use pretty_assertions::assert_eq;

fn run_fixture_test(fixture_name: &str) {
    // Create temporary directories
    let temp_in = tempfile::tempdir().unwrap();
    let temp_out = tempfile::tempdir().unwrap();

    // Path to the fixture and expected output
    let fixture_dir = format!("tests/fixtures/{}", fixture_name);
    let fixture_path = format!("{}/test_{}.lean", fixture_dir, fixture_name);
    let expected_path = format!("{}/expected_{}.md", fixture_dir, fixture_name);

    // Copy fixture file
    let fixture_content = fs::read_to_string(&fixture_path).unwrap_or_else(|_|
        panic!("Failed to read fixture: {}", fixture_path));
    fs::write(temp_in.path().join(format!("test_{}.lean", fixture_name)), fixture_content).unwrap();

    // Run the conversion
    lean2md::process_directory(temp_in.path(), temp_out.path()).unwrap_or_else(|e|
        panic!("Failed to process fixture {}: {}", fixture_name, e));

    // Read the actual output and normalize line endings
    let actual = fs::read_to_string(temp_out.path().join(format!("test_{}.md", fixture_name)))
        .unwrap_or_else(|_| panic!("Failed to read output for {}", fixture_name))
        .replace("\r\n", "\n");

    // Read the expected output and normalize line endings
    let expected = fs::read_to_string(&expected_path)
        .unwrap_or_else(|_| panic!("Failed to read expected output for {}", fixture_name))
        .replace("\r\n", "\n");

    // Compare with nice diff
    assert_eq!(actual.trim(), expected.trim(),
        "Test failed for {}", fixture_name);
}

#[test]
fn test_admonish() {
    run_fixture_test("admonish");
}

#[test]
fn test_docstrings() {
    run_fixture_test("docstrings");
}

#[test]
fn test_ignore_blocks() {
    run_fixture_test("ignore_blocks");
}

#[test]
fn test_markers() {
    run_fixture_test("markers");
}

#[test]
fn test_nested_code() {
    run_fixture_test("nested_code");
}
