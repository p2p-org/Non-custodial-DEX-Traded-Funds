import assert from 'assert';
import { PublicKey } from '@solana/web3.js';
import { complement, isNil } from 'ramda';
import { getConnection } from 'api/connection';
import { DTF_PROGRAM_ID } from 'config';
import { findFundsFx } from '..';
import { Fund } from '../../../../../js/lib/fund';

findFundsFx.use(async () => {
  const connection = getConnection();

  const filters = [
    {
      dataSize: 751,
    },
  ];

  const unsafeRes = await connection._rpcRequest('getProgramAccounts', [
    DTF_PROGRAM_ID.toBase58(),
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
        DTF_PROGRAM_ID.toBase58() +
        ': ' +
        res.error.message,
    );
  }

  const { result } = res;
  assert(typeof result !== 'undefined');

  return result
    .map((result: any) => {
      assert(result.account.data[1] === 'base64');

      const parsedData = Fund.decodePoolState(
        Buffer.from(result.account.data[0], 'base64'),
      );

      return {
        pubkey: new PublicKey(result.pubkey),
        account: {
          executable: result.account.executable,
          owner: new PublicKey(result.account.owner),
          lamports: result.account.lamports,
          data: parsedData,
        },
      };
    })
    .filter(complement(isNil));
});
