// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::ForkDataTestCase;
use ethereum_consensus::bellatrix::minimal as spec;
use ssz_rs::prelude::*;

#[test]
fn test_case_0() {
    let test_case = ForkDataTestCase::from(
        "../consensus-spec-tests/tests/minimal/bellatrix/ssz_static/ForkData/ssz_zero/case_0",
    );

    test_case.execute(|encoding| {
        let mut data: spec::ForkData = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}
