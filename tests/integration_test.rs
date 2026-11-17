// Integration test for SigmaOS-data-pseudonymization
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_data_pseudonymization as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
