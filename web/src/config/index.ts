// the default commitment uesd by the Solana web3 connection when checking the blockchain state
import { Commitment, PublicKey } from '@solana/web3.js';

export const isDev =
  process.env.NODE_ENV === 'development' || process.env.NODE_ENV === 'test';

export const cryptoCompareApiKey = process.env.REACT_APP_CRYPTO_COMPARE_API_KEY;

export const defaultCommitment: Commitment =
  (process.env.REACT_APP_DEFAULT_COMMITMENT as Commitment) || 'singleGossip';

// the amount of time to sleep after sending a transaction
// in order to work around a known solana web3 bug
export const postTransactionSleepMS = Number(
  process.env.REACT_APP_POST_TRANSACTION_SLEEP_MS,
);

export const DTF_PROGRAM_ID = new PublicKey(
  process.env.REACT_APP_DTF_PROGRAM_ID ||
    'HWEaSAnjKNADwA2ZopatwCM5BqrnMCPQBKAphf1s6XNW',
);

export const SWAP_PROGRAM_ID = new PublicKey(
  process.env.REACT_APP_SWAP_PROGRAM_ID ||
    'SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8',
);
