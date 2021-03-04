import { Numberu64, TokenSwapLayout } from '@solana/spl-token-swap';
import { PublicKey } from '@solana/web3.js';

export interface TokenSwapType {
  address: PublicKey;
  programId: PublicKey;
  tokenProgramId: PublicKey;
  poolToken: PublicKey;
  feeAccount: PublicKey;
  authority: PublicKey;
  tokenAccountA: PublicKey;
  tokenAccountB: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
  tradeFeeNumerator: Numberu64;
  tradeFeeDenominator: Numberu64;
  ownerTradeFeeNumerator: Numberu64;
  ownerTradeFeeDenominator: Numberu64;
  ownerWithdrawFeeNumerator: Numberu64;
  ownerWithdrawFeeDenominator: Numberu64;
  hostFeeNumerator: Numberu64;
  hostFeeDenominator: Numberu64;
  curveType: string;
}

export const parseTokenSwap = async (
  address: PublicKey,
  data: Buffer,
  programId: PublicKey,
): Promise<TokenSwapType> => {
  const tokenSwapData = TokenSwapLayout.decode(data);
  if (!tokenSwapData.isInitialized) {
    throw new Error(`Invalid token swap state`);
  }

  const [authority] = await PublicKey.findProgramAddress(
    [address.toBuffer()],
    programId,
  );

  const poolToken = new PublicKey(tokenSwapData.tokenPool);
  const feeAccount = new PublicKey(tokenSwapData.feeAccount);
  const tokenAccountA = new PublicKey(tokenSwapData.tokenAccountA);
  const tokenAccountB = new PublicKey(tokenSwapData.tokenAccountB);
  const mintA = new PublicKey(tokenSwapData.mintA);
  const mintB = new PublicKey(tokenSwapData.mintB);
  const tokenProgramId = new PublicKey(tokenSwapData.tokenProgramId);

  const tradeFeeNumerator = Numberu64.fromBuffer(
    tokenSwapData.tradeFeeNumerator,
  );
  const tradeFeeDenominator = Numberu64.fromBuffer(
    tokenSwapData.tradeFeeDenominator,
  );
  const ownerTradeFeeNumerator = Numberu64.fromBuffer(
    tokenSwapData.ownerTradeFeeNumerator,
  );
  const ownerTradeFeeDenominator = Numberu64.fromBuffer(
    tokenSwapData.ownerTradeFeeDenominator,
  );
  const ownerWithdrawFeeNumerator = Numberu64.fromBuffer(
    tokenSwapData.ownerWithdrawFeeNumerator,
  );
  const ownerWithdrawFeeDenominator = Numberu64.fromBuffer(
    tokenSwapData.ownerWithdrawFeeDenominator,
  );
  const hostFeeNumerator = Numberu64.fromBuffer(tokenSwapData.hostFeeNumerator);
  const hostFeeDenominator = Numberu64.fromBuffer(
    tokenSwapData.hostFeeDenominator,
  );
  const curveType = tokenSwapData.curveType;

  return {
    address,
    programId,
    tokenProgramId,
    poolToken,
    feeAccount,
    authority,
    tokenAccountA,
    tokenAccountB,
    mintA,
    mintB,
    tradeFeeNumerator,
    tradeFeeDenominator,
    ownerTradeFeeNumerator,
    ownerTradeFeeDenominator,
    ownerWithdrawFeeNumerator,
    ownerWithdrawFeeDenominator,
    hostFeeNumerator,
    hostFeeDenominator,
    curveType,
  };
};
