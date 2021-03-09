import {
  Account,
  Cluster,
  Commitment,
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionInstructionCtorFields,
} from '@solana/web3.js';

import nacl from 'tweetnacl';
import { postTransactionSleepMS } from 'config';
import { sleep } from 'utils/common';

import {
  confirmTransaction,
  DEFAULT_COMMITMENT,
  getConnection,
  getNetwork,
} from '../connection';
import { SolletWallet } from './SolletWallet';
import { Wallet, WalletEvent } from './Wallet';

const POST_TRANSACTION_SLEEP_MS = postTransactionSleepMS || 500;

/**
 * API for connecting to and interacting with a wallet
 */

// singleton wallet for the app.
// A user can be connected to only one wallet at a time.
let wallet: Wallet | null;
let connection: Connection | null;

const ERROR_CONNECT_FIRST = 'Connect first';

// eslint-disable-next-line no-shadow
export enum WalletType {
  MANUAL,
  SOLLET,
  BONFIDA,
  LOCAL,
}

const createWallet = (type: WalletType, cluster: Cluster): Wallet => {
  const network = getNetwork(cluster);
  switch (type) {
    case WalletType.SOLLET:
      return new SolletWallet(network);
    case WalletType.BONFIDA:
      return new SolletWallet(network, 'https://bonfida.com/wallet');
    default:
      return new SolletWallet(network);
  }
};

export const connect = async (
  cluster: Cluster,
  type: WalletType,
): Promise<Wallet> => {
  const newWallet = createWallet(type, cluster);

  // assign the singleton wallet.
  // Using a separate variable to simplify the type definitions
  wallet = newWallet;
  connection = getConnection(cluster);

  // connect is done once the wallet reports that it is connected.
  return new Promise((resolve) => {
    newWallet.on(WalletEvent.CONNECT, () => resolve(newWallet));
  });
};

export const disconnect = (): void => wallet?.disconnect();

export const makeTransaction = async (
  instructions: (TransactionInstruction | TransactionInstructionCtorFields)[],
  signers: Account[] = [],
): Promise<Transaction> => {
  if (!wallet || !connection) {
    throw new Error(ERROR_CONNECT_FIRST);
  }

  const { blockhash: recentBlockhash } = await connection.getRecentBlockhash();

  const signatures = [{ publicKey: wallet.pubkey }, ...signers];
  const transaction = new Transaction({
    recentBlockhash,
    signatures,
  });
  transaction.add(...instructions);

  // if there are any cosigners (other than the current wallet)
  // sign the transaction
  if (signers.length > 0) {
    transaction.partialSign(...signers);
  }

  return transaction;
};

interface SendOptions {
  commitment: Commitment;
  preflightCommitment: Commitment;
}

const defaultSendOptions = {
  commitment: DEFAULT_COMMITMENT,
  preflightCommitment: DEFAULT_COMMITMENT,
};

async function awaitConfirmation(
  signature: string,
  commitment:
    | 'processed'
    | 'confirmed'
    | 'finalized'
    | 'recent' // Deprecated as of v1.5.5
    | 'single' // Deprecated as of v1.5.5
    | 'singleGossip' // Deprecated as of v1.5.5
    | 'root' // Deprecated as of v1.5.5
    | 'max'
    | undefined,
) {
  console.log(`Submitted transaction ${signature}, awaiting confirmation`);
  await confirmTransaction(signature, commitment);
  console.log(`Transaction ${signature} confirmed`);

  if (wallet) {
    wallet.emit(WalletEvent.CONFIRMED, { transactionSignature: signature });
  }

  // workaround for a known solana web3 bug where
  // the state obtained from the http endpoint and the websocket are out of sync
  await sleep(POST_TRANSACTION_SLEEP_MS);
  return signature;
}

export const sendTransaction = async (
  transaction: Transaction,
  {
    commitment = defaultSendOptions.commitment,
    preflightCommitment = defaultSendOptions.preflightCommitment,
  }: Partial<SendOptions> = defaultSendOptions,
): Promise<string> => {
  if (!wallet || !connection) {
    throw new Error(ERROR_CONNECT_FIRST);
  }

  console.log('Sending signature request to wallet');
  const signed = await wallet.sign(transaction);
  console.log('Got signature, submitting transaction');

  for (const { signature, publicKey } of signed.signatures) {
    if (!signature) {
      console.log('1 false', signature, publicKey.toBase58(), transaction);
    } else if (
      !nacl.sign.detached.verify(
        signed.serializeMessage(),
        signature,
        publicKey.toBuffer(),
      )
    ) {
      console.log('2 false', signature, publicKey.toBase58(), transaction);
    }
  }

  console.log(111);

  const signature = await connection.sendRawTransaction(signed.serialize(), {
    preflightCommitment,
  });

  return awaitConfirmation(signature, commitment);
};

export const getWallet = (): Wallet => {
  if (!wallet || !connection) {
    throw new Error('notification.error.noWallet');
  }

  return wallet;
};

export const getBalance = (publicKey: PublicKey): Promise<number> => {
  if (!wallet || !connection) {
    throw new Error(ERROR_CONNECT_FIRST);
  }

  return connection.getBalance(publicKey);
};

export const getMinimumBalanceForRentExemption = (
  length: number,
): Promise<number> => {
  if (!wallet || !connection) {
    throw new Error(ERROR_CONNECT_FIRST);
  }

  return connection.getMinimumBalanceForRentExemption(length);
};
