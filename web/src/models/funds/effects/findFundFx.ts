import { path } from 'ramda';
import { getConnection } from 'api/connection';
import {
  PoolStatePopulated,
  TokenAccountPopulated,
  TokenPopulated,
} from 'models/types';
import { findFundFx } from '..';
import { Fund } from '../../../../../js/lib/fund';

findFundFx.use(async (fundAddress) => {
  const connection = getConnection();

  const result = await connection.getAccountInfo(fundAddress);

  if (!result) {
    throw new Error(`fund doesn't exists`);
  }

  const parsedData = Fund.decodePoolState(
    Buffer.from(result.data),
  ) as PoolStatePopulated;

  const addresses = [];
  parsedData.assets.forEach((asset) =>
    addresses.push(asset.vaultAddress.toBase58()),
  );
  parsedData.assets.forEach((asset) => addresses.push(asset.mint.toBase58()));

  addresses.push(parsedData.poolTokenMint.toBase58());

  const accountsResult = await connection._rpcRequest('getMultipleAccounts', [
    addresses,
    { commiment: connection.commitment, encoding: 'jsonParsed' },
  ]);

  parsedData.assets = parsedData.assets.map((asset, index) => {
    const vaultPopulated = path<TokenAccountPopulated>(
      ['result', 'value', index, 'data', 'parsed', 'info'],
      accountsResult,
    );

    const mintPopulated = path<TokenPopulated>(
      [
        'result',
        'value',
        parsedData.assets.length + index,
        'data',
        'parsed',
        'info',
      ],
      accountsResult,
    );

    return {
      ...asset,
      mintPopulated,
      vaultPopulated,
    };
  });

  parsedData.poolTokenMintPopulated = path<TokenPopulated>(
    ['result', 'value', parsedData.assets.length * 2, 'data', 'parsed', 'info'],
    accountsResult,
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
