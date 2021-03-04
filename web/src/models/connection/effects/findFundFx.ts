import { path } from 'ramda';
import { getConnection } from 'api/connection';
import { findFundFx } from '..';
import { Fund } from '../../../../../js/lib/fund';
import {
  PoolStatePopulated,
  TokenAccountPopulated,
  TokenPopulated,
} from '../types';

findFundFx.use(async (fundAddress) => {
  const connection = getConnection();

  const result = await connection.getAccountInfo(fundAddress);

  if (!result) {
    throw new Error(`fund doesn't exists`);
  }

  const parsedData = Fund.decodePoolState(
    Buffer.from(result.data),
  ) as PoolStatePopulated;

  const tokenAccounts = parsedData.assets
    .map((asset) => asset.vaultAddress.toBase58())
    .concat(parsedData.poolTokenMint.toBase58());

  const tokenAccountsResult = await connection._rpcRequest(
    'getMultipleAccounts',
    [
      tokenAccounts,
      { commiment: connection.commitment, encoding: 'jsonParsed' },
    ],
  );

  parsedData.assets = parsedData.assets.map((asset, index) => {
    const vaultPopulated = path<TokenAccountPopulated>(
      ['result', 'value', index, 'data', 'parsed', 'info'],
      tokenAccountsResult,
    );

    return {
      ...asset,
      vaultPopulated,
    };
  });

  parsedData.poolTokenMintPopulated = path<TokenPopulated>(
    ['result', 'value', parsedData.assets.length, 'data', 'parsed', 'info'],
    tokenAccountsResult,
  );

  return {
    pubkey: fundAddress,
    account: {
      executable: result.executable,
      owner: result.owner,
      lamports: result.lamports,
      data: parsedData,
    },
  };
});
