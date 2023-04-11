use std::collections::HashMap;
use std::io::Write;
use text_colorizer::*;

struct TestCase {
    formula: String,
    result: f64,
}

#[test]
fn test_calc() {
    let test_cases = get_testsaces();
    for (i, tc) in test_cases.iter().enumerate() {
        let mut vars: HashMap<String, f64> = HashMap::new();
        for line in tc.formula.split('\n') {
            match crate::process_form(line.replace("\r", ""), &mut vars) {
                Some(r) => {
                    if (r - tc.result).abs() < 0.0001 {
                        writeln!(
                            &mut std::io::stderr(),
                            "testcase {}: {} {}",
                            i,
                            "SUCCESSED          ".green(),
                            tc.formula
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            &mut std::io::stderr(),
                            "testcase {}: {} {} = {}, but expected {}",
                            i,
                            "CALCULATION FAILED ".red(),
                            tc.formula,
                            r,
                            tc.result
                        )
                        .unwrap();
                    }
                }
                None => {
                    writeln!(
                        &mut std::io::stderr(),
                        "testcase {}: {} {}",
                        i,
                        "PARSE FAILD        ".red(),
                        tc.formula
                    )
                    .unwrap();
                }
            }
        }
    }
}

fn get_testsaces() -> Vec<TestCase> {
    let mut test_cases: Vec<TestCase> = Vec::new();
    test_cases.push(TestCase {
        formula: "3+3".to_string(),
        result: 6.0,
    });
    test_cases.push(TestCase {
        formula: "\\frac {1}{2}".to_string(),
        result: 0.5,
    });
    test_cases.push(TestCase {
        formula: "-\\abs (-2)^{\\frac{1}{4/2}}^{6}".to_string(),
        result: -8.0,
    });
    test_cases
}
