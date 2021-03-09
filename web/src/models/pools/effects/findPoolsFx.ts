import assert from 'assert';
import { PublicKey } from '@solana/web3.js';
import { complement, isNil, path } from 'ramda';
import { TokenSwapLayout } from '@solana/spl-token-swap';
import { getConnection } from 'api/connection';
import { SWAP_PROGRAM_ID } from 'config';
import { parseTokenSwap } from '../../connection/layouts/tokenSwap';
import { findPoolsFx } from '..';
import {
  TokenAccountPopulated,
  TokenPopulated,
  TokenSwapPopulated,
} from '../../types';

findPoolsFx.use(async () => {
  const connection = getConnection();

  const filters = [
    {
      dataSize: TokenSwapLayout.span,
    },
  ];

  const unsafeRes = await connection._rpcRequest('getProgramAccounts', [
    SWAP_PROGRAM_ID.toBase58(),
    {
      commitment: connection.commitment,
      filters,
      encoding: 'base64',
    },
  ]);

  const res = unsafeRes;
  if (res.error) {
    throw new Error(
      'failed to get accounts owned by program ' +
        SWAP_PROGRAM_ID.toBase58() +
        ': ' +
        res.error.message,
    );
  }

  const { result } = res;
  assert(typeof result !== 'undefined');

  const tokenSwapPromises = result.map(async (result: any) => {
    assert(result.account.data[1] === 'base64');

    const tokenSwap = (await parseTokenSwap(
      new PublicKey(result.pubkey),
      Buffer.from(result.account.data[0], 'base64'),
      SWAP_PROGRAM_ID,
    )) as TokenSwapPopulated;

    // TODO: multiple
    const tokenAccountAResult = await connection.getParsedAccountInfo(
      tokenSwap.tokenAccountA,
    );
    tokenSwap.tokenAccountAPopulated = path<TokenAccountPopulated>(
      ['value', 'data', 'parsed', 'info'],
      tokenAccountAResult,
    );

    const tokenAccountBResult = await connection.getParsedAccountInfo(
      tokenSwap.tokenAccountB,
    );
    tokenSwap.tokenAccountBPopulated = path<TokenAccountPopulated>(
      ['value', 'data', 'parsed', 'info'],
      tokenAccountBResult,
    );

    const poolTokenResult = await connection.getParsedAccountInfo(
      tokenSwap.poolToken,
    );
    tokenSwap.poolTokenPopulated = path<TokenPopulated>(
      ['value', 'data', 'parsed', 'info'],
      poolTokenResult,
    );

    return tokenSwap;
  });

  const tokenSwaps = await Promise.all(tokenSwapPromises);

  return tokenSwaps.filter(complement(isNil)) as TokenSwapPopulated[];
});
