import assert from 'assert';
import { PublicKey } from '@solana/web3.js';
import { complement, isNil } from 'ramda';
import { TokenSwapLayout } from '@solana/spl-token-swap';
import { getConnection } from 'api/connection';
import { SWAP_PROGRAM_ID } from 'config';
import { GetProgramAccountsRpcResult } from '../structs';
import { findPoolsFx } from '..';
import { parseTokenSwap, TokenSwapType } from '../layouts/tokenSwap';

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

  const res = GetProgramAccountsRpcResult(unsafeRes);
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

    return parseTokenSwap(
      new PublicKey(result.pubkey),
      Buffer.from(result.account.data[0], 'base64'),
      SWAP_PROGRAM_ID,
    );
  });

  const tokenSwaps = await Promise.all(tokenSwapPromises);

  return tokenSwaps.filter(complement(isNil)) as TokenSwapType[];
});
