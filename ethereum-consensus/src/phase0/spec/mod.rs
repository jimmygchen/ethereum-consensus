//! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
pub use crate::{
    phase0::{
        beacon_block::{
            BeaconBlock, BeaconBlockBody, BeaconBlockHeader, SignedBeaconBlock,
            SignedBeaconBlockHeader,
        },
        beacon_state::{BeaconState, HistoricalBatch, HistoricalSummary},
        block_processing::{
            get_validator_from_deposit, process_attestation, process_attester_slashing,
            process_block, process_block_header, process_deposit, process_eth1_data,
            process_operations, process_proposer_slashing, process_randao, process_voluntary_exit,
            xor,
        },
        constants::{
            BASE_REWARDS_PER_EPOCH, DEPOSIT_CONTRACT_TREE_DEPTH, DEPOSIT_DATA_LIST_BOUND,
            JUSTIFICATION_BITS_LENGTH,
        },
        epoch_processing::{
            get_attestation_component_deltas, get_attestation_deltas, get_attesting_balance,
            get_base_reward, get_finality_delay, get_head_deltas, get_inactivity_penalty_deltas,
            get_inclusion_delay_deltas, get_matching_head_attestations,
            get_matching_source_attestations, get_matching_target_attestations,
            get_proposer_reward, get_source_deltas, get_target_deltas,
            get_unslashed_attesting_indices, is_in_inactivity_leak,
            process_effective_balance_updates, process_epoch, process_eth1_data_reset,
            process_historical_roots_update, process_justification_and_finalization,
            process_participation_record_updates, process_randao_mixes_reset,
            process_registry_updates, process_rewards_and_penalties, process_slashings,
            process_slashings_reset, weigh_justification_and_finalization,
        },
        fork::{Fork, ForkData},
        genesis::{get_genesis_block, initialize_beacon_state_from_eth1, is_valid_genesis_state},
        helpers::{
            compute_activation_exit_epoch, compute_committee, compute_domain,
            compute_epoch_at_slot, compute_fork_data_root, compute_fork_digest,
            compute_proposer_index, compute_shuffled_index, compute_start_slot_at_epoch,
            decrease_balance, get_active_validator_indices, get_attesting_indices,
            get_beacon_committee, get_beacon_proposer_index, get_block_root,
            get_block_root_at_slot, get_committee_count_per_slot, get_current_epoch, get_domain,
            get_eligible_validator_indices, get_indexed_attestation, get_previous_epoch,
            get_randao_mix, get_seed, get_total_active_balance, get_total_balance,
            get_validator_churn_limit, increase_balance, initiate_validator_exit,
            is_active_validator, is_eligible_for_activation, is_eligible_for_activation_queue,
            is_slashable_attestation_data, is_slashable_validator, is_valid_indexed_attestation,
            slash_validator, verify_block_signature,
        },
        operations::{
            Attestation, AttestationData, AttesterSlashing, Checkpoint, Deposit, DepositData,
            DepositMessage, Eth1Data, IndexedAttestation, PendingAttestation, ProposerSlashing,
            SignedVoluntaryExit, VoluntaryExit,
        },
        slot_processing::{process_slot, process_slots},
        state_transition::{state_transition, state_transition_block_in_slot},
        validator::{AggregateAndProof, Eth1Block, SignedAggregateAndProof, Validator},
    },
    primitives::*,
    signing::*,
    state_transition::*,
};
