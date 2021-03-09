import { PublicKey } from '@solana/web3.js';
import { getConnection } from 'api/connection';
import { getWallet } from 'api/wallet';
import { TOKEN_PROGRAM_ID } from 'config';
import { getParsedTokenAccountsByOwnerFx } from '..';
import { TokenAccountType } from '../../types';

getParsedTokenAccountsByOwnerFx.use(async () => {
  const connection = getConnection();
  const wallet = getWallet().pubkey;

  const result = await connection.getParsedTokenAccountsByOwner(wallet, {
    programId: TOKEN_PROGRAM_ID,
  });

  return result.value.map((account) => ({
    pubkey: account.pubkey,
    account: {
      executable: account.account.executable,
      owner: account.account.owner,
      lamports: account.account.lamports,
      data: account.account.data.parsed.info,
    },
  })) as TokenAccountType[];
});
