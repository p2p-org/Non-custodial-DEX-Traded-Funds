import { PoolState } from '../../../../js/lib/fund';

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

interface PoolStatePopulated extends PoolState {
  assets: {
    mint: PublicKey;
    vaultAddress: PublicKey;
    vaultPopulated?: TokenAccountPopulated;
  }[];
  poolTokenMintPopulated?: TokenPopulated;
}
