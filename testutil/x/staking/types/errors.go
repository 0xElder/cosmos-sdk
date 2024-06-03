package types

import "cosmossdk.io/errors"

// x/staking module sentinel errors
var (
	ErrNoValidatorFound                = errors.Register(ModuleName, 3, "validator does not exist")
	ErrValidatorOwnerExists            = errors.Register(ModuleName, 4, "validator already exist for this operator address; must use new validator operator address")
	ErrValidatorPubKeyExists           = errors.Register(ModuleName, 5, "validator already exist for this pubkey; must use new validator pubkey")
	ErrValidatorPubKeyTypeNotSupported = errors.Register(ModuleName, 6, "validator pubkey type is not supported")
	ErrCommissionNegative              = errors.Register(ModuleName, 9, "commission must be positive")
	ErrCommissionHuge                  = errors.Register(ModuleName, 10, "commission cannot be more than 100%")
	ErrCommissionGTMaxRate             = errors.Register(ModuleName, 11, "commission cannot be more than the max rate")
	ErrCommissionUpdateTime            = errors.Register(ModuleName, 12, "commission cannot be changed more than once in 24h")
	ErrCommissionChangeRateNegative    = errors.Register(ModuleName, 13, "commission change rate must be positive")
	ErrCommissionChangeRateGTMaxRate   = errors.Register(ModuleName, 14, "commission change rate cannot be more than the max rate")
	ErrCommissionGTMaxChangeRate       = errors.Register(ModuleName, 15, "commission cannot be changed more than max change rate")
	ErrSelfDelegationBelowMinimum      = errors.Register(ModuleName, 16, "validator's self delegation must be greater than their minimum self delegation")
	ErrInsufficientShares              = errors.Register(ModuleName, 22, "insufficient delegation shares")
	ErrNotEnoughDelegationShares       = errors.Register(ModuleName, 24, "not enough delegation shares")
	ErrNotMature                       = errors.Register(ModuleName, 25, "entry not mature")
	ErrNoUnbondingDelegation           = errors.Register(ModuleName, 26, "no unbonding delegation found")
	ErrMaxUnbondingDelegationEntries   = errors.Register(ModuleName, 27, "too many unbonding delegation entries for (delegator, validator) tuple")
	ErrNoRedelegation                  = errors.Register(ModuleName, 28, "no redelegation found")
	ErrSelfRedelegation                = errors.Register(ModuleName, 29, "cannot redelegate to the same validator")
	ErrTinyRedelegationAmount          = errors.Register(ModuleName, 30, "too few tokens to redelegate (truncates to zero tokens)")
	ErrBadRedelegationDst              = errors.Register(ModuleName, 31, "redelegation destination validator not found")
	ErrTransitiveRedelegation          = errors.Register(ModuleName, 32, "redelegation to this validator already in progress; first redelegation to this validator must complete before next redelegation")
	ErrMaxRedelegationEntries          = errors.Register(ModuleName, 33, "too many redelegation entries for (delegator, src-validator, dst-validator) tuple")
	ErrDelegatorShareExRateInvalid     = errors.Register(ModuleName, 34, "cannot delegate to validators with invalid (zero) ex-rate")
	ErrBothShareMsgsGiven              = errors.Register(ModuleName, 35, "both shares amount and shares percent provided")
	ErrNeitherShareMsgsGiven           = errors.Register(ModuleName, 36, "neither shares amount nor shares percent provided")
	ErrInvalidHistoricalInfo           = errors.Register(ModuleName, 37, "invalid historical info")
	ErrEmptyValidatorPubKey            = errors.Register(ModuleName, 39, "empty validator public key")
	ErrCommissionLTMinRate             = errors.Register(ModuleName, 40, "commission cannot be less than min rate")
	ErrUnbondingNotFound               = errors.Register(ModuleName, 41, "unbonding operation not found")
	ErrUnbondingOnHoldRefCountNegative = errors.Register(ModuleName, 42, "cannot un-hold unbonding operation that is not on hold")
	ErrInvalidSigner                   = errors.Register(ModuleName, 43, "expected authority account as only signer for proposal message")
	ErrBadRedelegationSrc              = errors.Register(ModuleName, 44, "redelegation source validator not found")
	ErrNoUnbondingType                 = errors.Register(ModuleName, 45, "unbonding type not found")

	// consensus key errors
	ErrConsensusPubKeyAlreadyUsedForValidator = errors.Register(ModuleName, 46, "consensus pubkey is already used for a validator")
	ErrExceedingMaxConsPubKeyRotations        = errors.Register(ModuleName, 47, "exceeding maximum consensus pubkey rotations within unbonding period")
	ErrConsensusPubKeyLenInvalid              = errors.Register(ModuleName, 48, "consensus pubkey len is invalid")
)
