// the default commitment uesd by the Solana web3 connection when checking the blockchain state
import { Commitment } from '@solana/web3.js';

export const isDev =
  process.env.NODE_ENV === 'development' || process.env.NODE_ENV === 'test';

export const defaultCommitment: Commitment =
  (process.env.DEFAULT_COMMITMENT as Commitment) || 'singleGossip';

// the amount of time to sleep after sending a transaction
// in order to work around a known solana web3 bug
export const postTransactionSleepMS = Number(
  process.env.POST_TRANSACTION_SLEEP_MS,
);
