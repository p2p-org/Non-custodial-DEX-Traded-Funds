import { PublicKey, PublicKeyAndAccount } from '@solana/web3.js';
import { PoolState } from '../../../js/lib/fund';
import { TokenSwapType } from './connection/layouts/tokenSwap';

interface TokenAccountPopulated {
  isNative: boolean;
  mint: string;
  owner: string;
  state: string;
  tokenAmount: {
    amount: string;
    decimals: number;
    uiAmount: number;
  };
}

interface TokenPopulated {
  decimals: number;
  freezeAuthority: string | null;
  isInitialized: boolean;
  mintAuthority: string;
  supply: string;
}

interface AssetType {
  mint: PublicKey;
  mintPopulated?: TokenPopulated;
  vaultAddress: PublicKey;
  vaultPopulated?: TokenAccountPopulated;
}

interface PoolStatePopulated extends PoolState {
  assets: AssetType[];
  poolTokenMintPopulated?: TokenPopulated;
}

type FundType = PublicKeyAndAccount<PoolStatePopulated>;
type TokenAccountType = PublicKeyAndAccount<TokenAccountPopulated>;

interface TokenSwapPopulated extends TokenSwapType {
  tokenAccountAPopulated?: TokenAccountPopulated;
  tokenAccountBPopulated?: TokenAccountPopulated;
  poolTokenPopulated?: TokenPopulated;
}
