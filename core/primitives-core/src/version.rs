use crate::types::ProtocolVersion;

/// New Protocol features should go here. Features are guarded by their corresponding feature flag.
/// For example, if we have `ProtocolFeature::EVM` and a corresponding feature flag `evm`, it will look
/// like
///
/// #[cfg(feature = "protocol_feature_evm")]
/// EVM code
///
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ProtocolFeature {
    // stable features
    ImplicitAccountCreation,
    RectifyInflation,
    /// Add `AccessKey` nonce range by setting nonce to `(block_height - 1) * 1e6`, see
    /// <https://github.com/near/nearcore/issues/3779>.
    AccessKeyNonceRange,
    /// Don't process any receipts for shard when chunk is not present.
    /// Always use gas price computed in the previous block.
    FixApplyChunks,
    LowerStorageCost,
    DeleteActionRestriction,
    /// Add versions to `Account` data structure
    AccountVersions,
    TransactionSizeLimit,
    /// Fix a bug in `storage_usage` for account caused by #3824
    FixStorageUsage,
    /// Cap maximum gas price to 2,000,000,000 yoctoNEAR
    CapMaxGasPrice,
    CountRefundReceiptsInGasLimit,
    /// Add `ripemd60` and `ecrecover` host function
    MathExtension,
    /// Restore receipts that were previously stuck because of
    /// <https://github.com/near/nearcore/pull/4228>.
    RestoreReceiptsAfterFixApplyChunks,
    /// This feature switch our WASM engine implementation from wasmer 0.* to
    /// wasmer 2.*, bringing better performance and reliability.
    ///
    /// The implementations should be sufficiently similar for this to not be a
    /// protocol upgrade, but we conservatively do a protocol upgrade to be on
    /// the safe side.
    ///
    /// Although wasmer2 is faster, we don't change fees with this protocol
    /// version -- we can safely do that in a separate step.
    Wasmer2,
    SimpleNightshade,
    LowerDataReceiptAndEcrecoverBaseCost,
    /// Lowers the cost of wasm instruction due to switch to wasmer2.
    LowerRegularOpCost,
    /// Lowers the cost of wasm instruction due to switch to faster,
    /// compiler-intrinsics based gas counter.
    LowerRegularOpCost2,
    /// Limit number of wasm functions in one contract. See
    /// <https://github.com/near/nearcore/pull/4954> for more details.
    LimitContractFunctionsNumber,
    BlockHeaderV3,
    /// Changes how we select validators for epoch and how we select validators
    /// within epoch.  See <https://github.com/near/NEPs/pull/167> for general
    /// description, note that we would not introduce chunk-only validators with
    /// this feature
    AliasValidatorSelectionAlgorithm,
    /// Make block producers produce chunks for the same block they would later produce to avoid
    /// network delays
    SynchronizeBlockChunkProduction,
    /// Change the algorithm to count WASM stack usage to avoid undercounting in
    /// some cases.
    CorrectStackLimit,
    /// Add `AccessKey` nonce range for implicit accounts, as in `AccessKeyNonceRange` feature.
    AccessKeyNonceForImplicitAccounts,
    /// Increase cost per deployed code byte to cover for the compilation steps
    /// that a deployment triggers. Only affects the action execution cost.
    IncreaseDeploymentCost,
    FunctionCallWeight,
    /// This feature enforces a global limit on the function local declarations in a WebAssembly
    /// contract. See <...> for more information.
    LimitContractLocals,
    /// Ensure caching all nodes in the chunk for which touching trie node cost was charged. Charge for each such node
    /// only once per chunk at the first access time.
    ChunkNodesCache,
    /// Lower `max_length_storage_key` limit, which itself limits trie node sizes.
    LowerStorageKeyLimit,
    // alt_bn128_g1_multiexp, alt_bn128_g1_sum, alt_bn128_pairing_check host functions
    AltBn128,
    ChunkOnlyProducers,
    /// Ensure the total stake of validators that are kicked out does not exceed a percentage of total stakes
    MaxKickoutStake,
    /// Validate account id for function call access keys.
    AccountIdInFunctionCallPermission,
    /// Zero Balance Account NEP 448: <https://github.com/near/NEPs/pull/448>
    ZeroBalanceAccount,
    /// Execute a set of actions on behalf of another account.
    ///
    /// Meta Transaction NEP-366: <https://github.com/near/NEPs/blob/master/neps/nep-0366.md>
    DelegateAction,
    Ed25519Verify,
    /// Decouple compute and gas costs of operations to safely limit the compute time it takes to
    /// process the chunk.
    ///
    /// Compute Costs NEP-455: <https://github.com/near/NEPs/blob/master/neps/nep-0455.md>
    ComputeCosts,
    /// Decrease the cost of function call action. Only affects the execution cost.
    DecreaseFunctionCallBaseCost,
    /// Enable flat storage for reads, reducing number of DB accesses from `2 * key.len()` in
    /// the worst case to 2.
    ///
    /// Flat Storage NEP-399: <https://github.com/near/NEPs/blob/master/neps/nep-0399.md>
    FlatStorageReads,
    /// Enables preparation V2. Note that this setting is not supported in production settings
    /// without NearVmRuntime enabled alongside it, as the VM runner would be too slow.
    PreparationV2,
    /// Enables Near-Vm. Note that this setting is not at all supported without PreparationV2,
    /// as it hardcodes preparation v2 code into the generated assembly.
    NearVmRuntime,
    BlockHeaderV4,
    /// Resharding V2. A new implementation for resharding and a new shard
    /// layout for the production networks.
    SimpleNightshadeV2,
    /// Built on top of Resharding V2. Changes shard layout to V3 to split shard 2 into two parts.
    SimpleNightshadeV3,
    /// In case not all validator seats are occupied our algorithm provide incorrect minimal seat
    /// price - it reports as alpha * sum_stake instead of alpha * sum_stake / (1 - alpha), where
    /// alpha is min stake ratio
    #[cfg(feature = "protocol_feature_fix_staking_threshold")]
    FixStakingThreshold,
    /// Charge for contract loading before it happens.
    #[cfg(feature = "protocol_feature_fix_contract_loading_cost")]
    FixContractLoadingCost,
    #[cfg(feature = "protocol_feature_reject_blocks_with_outdated_protocol_version")]
    RejectBlocksWithOutdatedProtocolVersions,
    /// Allows creating an account with a non refundable balance to cover storage costs.
    /// NEP: <https://github.com/near/NEPs/pull/491>
    #[cfg(feature = "protocol_feature_nonrefundable_transfer_nep491")]
    NonrefundableStorage,
    // NEP: https://github.com/near/NEPs/pull/488
    #[cfg(feature = "protocol_feature_bls12381")]
    BLS12381,
    RestrictTla,
    /// Increases the number of chunk producers.
    TestnetFewerBlockProducers,
    /// Enables stateless validation which is introduced in <https://github.com/near/NEPs/pull/509>
    StatelessValidationV0,
    EthImplicitAccounts,
    /// Enables yield execution which is introduced in <https://github.com/near/NEPs/pull/519>
    YieldExecution,

    /// Protocol version reserved for use in resharding tests.
    SimpleNightshadeTestonly,

    // Stateless validation: lower block and chunk validator kickout percent from 90 to 50.
    LowerValidatorKickoutPercentForDebugging,
    // Stateless validation: single shard tracking.
    SingleShardTracking,
    // Stateless validation: state witness size limits.
    StateWitnessSizeLimit,
    // Shuffle shard assignments for chunk producers at every epoch.
    ShuffleShardAssignments,
    // Stateless validation: limit the size of storage proof generated by a single receipt.
    // Receipts which generate storage proofs larger than this limit will be rejected.
    // Protocol 85 also decreased the soft per-chunk storage proof limit to 3MB.
    PerReceiptHardStorageProofLimit,
    /// Cross-shard congestion control according to <https://github.com/near/NEPs/pull/539>.
    CongestionControl,
    /// Remove account with long storage key.
    RemoveAccountWithLongStorageKey,
    // Stateless validation: Distribute state witness as reed solomon encoded parts
    PartialEncodedStateWitness,
    /// Size limits for transactions included in a ChunkStateWitness.
    WitnessTransactionLimits,
    /// Size limit on outgoing receipts.
    OutgoingReceiptsSizeLimit,
    /// No chunk-only producers in stateless validation
    NoChunkOnlyProducers,
    /// Decrease the ratio of data parts in the Reed Solomon encoding for partial witness distribution.
    ChangePartialWitnessDataPartsRequired,
    /// Increase the `combined_transactions_size_limit` to 4MiB to allow higher throughput.
    BiggerCombinedTransactionLimit,
    /// Increase gas cost of sending receipt to another account to 50 TGas / MiB
    HigherSendingCost,
}

impl ProtocolFeature {
    pub const fn protocol_version(self) -> ProtocolVersion {
        match self {
            // Stable features
            ProtocolFeature::ImplicitAccountCreation => 35,
            ProtocolFeature::LowerStorageCost => 42,
            ProtocolFeature::DeleteActionRestriction => 43,
            ProtocolFeature::FixApplyChunks => 44,
            ProtocolFeature::RectifyInflation | ProtocolFeature::AccessKeyNonceRange => 45,
            ProtocolFeature::AccountVersions
            | ProtocolFeature::TransactionSizeLimit
            | ProtocolFeature::FixStorageUsage
            | ProtocolFeature::CapMaxGasPrice
            | ProtocolFeature::CountRefundReceiptsInGasLimit
            | ProtocolFeature::MathExtension => 46,
            ProtocolFeature::RestoreReceiptsAfterFixApplyChunks => 47,
            ProtocolFeature::Wasmer2
            | ProtocolFeature::LowerDataReceiptAndEcrecoverBaseCost
            | ProtocolFeature::LowerRegularOpCost
            | ProtocolFeature::SimpleNightshade => 48,
            ProtocolFeature::LowerRegularOpCost2
            | ProtocolFeature::LimitContractFunctionsNumber
            | ProtocolFeature::BlockHeaderV3
            | ProtocolFeature::AliasValidatorSelectionAlgorithm => 49,
            ProtocolFeature::SynchronizeBlockChunkProduction
            | ProtocolFeature::CorrectStackLimit => 50,
            ProtocolFeature::AccessKeyNonceForImplicitAccounts => 51,
            ProtocolFeature::IncreaseDeploymentCost
            | ProtocolFeature::FunctionCallWeight
            | ProtocolFeature::LimitContractLocals
            | ProtocolFeature::ChunkNodesCache
            | ProtocolFeature::LowerStorageKeyLimit => 53,
            ProtocolFeature::AltBn128 => 55,
            ProtocolFeature::ChunkOnlyProducers | ProtocolFeature::MaxKickoutStake => 56,
            ProtocolFeature::AccountIdInFunctionCallPermission => 57,
            ProtocolFeature::Ed25519Verify
            | ProtocolFeature::ZeroBalanceAccount
            | ProtocolFeature::DelegateAction => 59,
            ProtocolFeature::ComputeCosts | ProtocolFeature::FlatStorageReads => 61,
            ProtocolFeature::PreparationV2 | ProtocolFeature::NearVmRuntime => 62,
            ProtocolFeature::BlockHeaderV4 => 63,
            ProtocolFeature::RestrictTla
            | ProtocolFeature::TestnetFewerBlockProducers
            | ProtocolFeature::SimpleNightshadeV2 => 64,
            ProtocolFeature::SimpleNightshadeV3 => 65,
            ProtocolFeature::DecreaseFunctionCallBaseCost => 66,
            ProtocolFeature::YieldExecution => 67,
            ProtocolFeature::CongestionControl
            | ProtocolFeature::RemoveAccountWithLongStorageKey => 68,
            // Stateless validation features.
            // TODO All of the stateless validation features should be collapsed
            // into a single protocol feature.
            ProtocolFeature::StatelessValidationV0
            | ProtocolFeature::LowerValidatorKickoutPercentForDebugging
            | ProtocolFeature::SingleShardTracking
            | ProtocolFeature::StateWitnessSizeLimit
            | ProtocolFeature::PerReceiptHardStorageProofLimit
            | ProtocolFeature::PartialEncodedStateWitness
            | ProtocolFeature::WitnessTransactionLimits
            | ProtocolFeature::OutgoingReceiptsSizeLimit
            | ProtocolFeature::NoChunkOnlyProducers
            | ProtocolFeature::ChangePartialWitnessDataPartsRequired
            | ProtocolFeature::BiggerCombinedTransactionLimit
            | ProtocolFeature::HigherSendingCost => 69,
            ProtocolFeature::EthImplicitAccounts => 70,

            // This protocol version is reserved for use in resharding tests. An extra resharding
            // is simulated on top of the latest shard layout in production. Note that later
            // protocol versions will still have the production layout.
            ProtocolFeature::SimpleNightshadeTestonly => 100,

            // Nightly features
            #[cfg(feature = "protocol_feature_fix_staking_threshold")]
            ProtocolFeature::FixStakingThreshold => 126,
            #[cfg(feature = "protocol_feature_fix_contract_loading_cost")]
            ProtocolFeature::FixContractLoadingCost => 129,
            #[cfg(feature = "protocol_feature_reject_blocks_with_outdated_protocol_version")]
            ProtocolFeature::RejectBlocksWithOutdatedProtocolVersions => 132,
            #[cfg(feature = "protocol_feature_nonrefundable_transfer_nep491")]
            ProtocolFeature::NonrefundableStorage => 140,
            #[cfg(feature = "protocol_feature_bls12381")]
            ProtocolFeature::BLS12381 => 141,
            // TODO(#11201): When stabilizing this feature in mainnet, also remove the temporary code
            // that always enables this for mocknet (see config_mocknet function).
            ProtocolFeature::ShuffleShardAssignments => 143,
        }
    }

    pub fn enabled(&self, protocol_version: ProtocolVersion) -> bool {
        protocol_version >= self.protocol_version()
    }
}

/// Current protocol version used on the mainnet.
/// Some features (e. g. FixStorageUsage) require that there is at least one epoch with exactly
/// the corresponding version
const STABLE_PROTOCOL_VERSION: ProtocolVersion = 70;

/// Largest protocol version supported by the current binary.
pub const PROTOCOL_VERSION: ProtocolVersion = if cfg!(feature = "statelessnet_protocol") {
    // Please note that congestion control and stateless validation are now
    // stabilized but statelessnet should remain at its own version.
    82
} else if cfg!(feature = "nightly_protocol") {
    // On nightly, pick big enough version to support all features.
    143
} else {
    // Enable all stable features.
    STABLE_PROTOCOL_VERSION
};

/// Both, outgoing and incoming tcp connections to peers, will be rejected if `peer's`
/// protocol version is lower than this.
pub const PEER_MIN_ALLOWED_PROTOCOL_VERSION: ProtocolVersion = STABLE_PROTOCOL_VERSION - 3;

#[macro_export]
macro_rules! checked_feature {
    ("stable", $feature:ident, $current_protocol_version:expr) => {{
        $crate::version::ProtocolFeature::$feature.protocol_version() <= $current_protocol_version
    }};
    ($feature_name:tt, $feature:ident, $current_protocol_version:expr) => {{
        #[cfg(feature = $feature_name)]
        let is_feature_enabled = $crate::version::ProtocolFeature::$feature.protocol_version()
            <= $current_protocol_version;
        #[cfg(not(feature = $feature_name))]
        let is_feature_enabled = {
            // Workaround unused variable warning
            let _ = $current_protocol_version;

            false
        };
        is_feature_enabled
    }};

    ($feature_name:tt, $feature:ident, $current_protocol_version:expr, $feature_block:block) => {{
        checked_feature!($feature_name, $feature, $current_protocol_version, $feature_block, {})
    }};

    ($feature_name:tt, $feature:ident, $current_protocol_version:expr, $feature_block:block, $non_feature_block:block) => {{
        #[cfg(feature = $feature_name)]
        {
            if checked_feature!($feature_name, $feature, $current_protocol_version) {
                $feature_block
            } else {
                $non_feature_block
            }
        }
        // Workaround unused variable warning
        #[cfg(not(feature = $feature_name))]
        {
            let _ = $current_protocol_version;
            $non_feature_block
        }
    }};
}
