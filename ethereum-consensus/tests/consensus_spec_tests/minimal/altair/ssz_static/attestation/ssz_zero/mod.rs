// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::AttestationTestCase;
use ethereum_consensus::altair::minimal as spec;
use ssz_rs::prelude::*;

#[test]
fn test_case_0() {
    let test_case = AttestationTestCase::from(
        "../consensus-spec-tests/tests/minimal/altair/ssz_static/Attestation/ssz_zero/case_0",
    );

    test_case.execute(|encoding| {
        let mut data: spec::Attestation = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}
