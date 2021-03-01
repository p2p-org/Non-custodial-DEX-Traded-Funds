import { PublicKey } from '@solana/web3.js';
import { getConnection } from 'api/connection';
import { findFundFx } from '..';
import { Fund } from '../../../../../js/lib/fund';

findFundFx.use(async () => {
  const connection = getConnection();
  const pubkey = new PublicKey('t72redTRJkPtUmTWWvPyjnkFKGVrHakv3DBQTheY4oD');

  const result = await connection.getAccountInfo(pubkey);

  if (!result) {
    throw new Error(`fund doesn't exists`);
  }

  const parsedData = Fund.decodePoolState(Buffer.from(result.data));

  return {
    pubkey,
    account: {
      executable: result.executable,
      owner: result.owner,
      lamports: result.lamports,
      data: parsedData,
    },
  };
});
