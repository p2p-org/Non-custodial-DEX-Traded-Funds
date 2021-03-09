import { AccountLayout, Token as SPLToken, u64 } from '@solana/spl-token';
import {
  Account,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import nacl from 'tweetnacl';
import { Numberu64, TokenSwap } from '@solana/spl-token-swap';
import { Decimal } from 'decimal.js';
import BN from 'bn.js';
import { getWallet, makeTransaction, sendTransaction } from 'api/wallet';
import {
  DTF_PROGRAM_ID,
  HOST_FEE_VAULT,
  SWAP_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from 'config';
import { FundType, TokenSwapPopulated } from 'models/types';
import { getConnection } from 'api/connection';
import { TokenSwapType } from 'models/connection/layouts/tokenSwap';
import { Fund as FUND } from '../../../../js/lib/fund';

const createAccountByMint = async (
  owner: PublicKey,
  token: PublicKey,
  instructions: TransactionInstruction[],
  cleanupInstructions: TransactionInstruction[],
  signers: Account[],
) => {
  const connection = getConnection();
  const newAccount = new Account();

  const accountRentExempt = await connection.getMinimumBalanceForRentExemption(
    AccountLayout.span,
  );

  instructions.push(
    SystemProgram.createAccount({
      fromPubkey: getWallet().pubkey,
      newAccountPubkey: newAccount.publicKey,
      lamports: accountRentExempt,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    }),
  );

  instructions.push(
    SPLToken.createInitAccountInstruction(
      TOKEN_PROGRAM_ID,
      token,
      newAccount.publicKey,
      owner,
    ),
  );

  signers.push(newAccount);

  return newAccount.publicKey;
};

// const createSwapTransactionInstruction = (
//     parameters: Required<SwapParameters> & {
//       hostFeePublicKey?: PublicKey;
//     },
// ): TransactionInstruction => {
//   const isReverse = isReverseSwap(parameters);
//   const poolIntoAccount = isReverse ? parameters.pool.tokenB : parameters.pool.tokenA;
//   const poolFromAccount = isReverse ? parameters.pool.tokenA : parameters.pool.tokenB;
//
//   // handle slippage by setting a minimum expected TO amount
//   // the transaction will fail if the received amount is lower than this.
//   const minimumToAmountWithoutSlippage = parameters.pool.calculateAmountInOtherToken(
//       parameters.fromAccount.mint,
//       parameters.fromAmount,
//       true,
//   );
//
//   const minimumToAmountWithSlippage = adjustForSlippage(
//       minimumToAmountWithoutSlippage,
//       'down',
//       parameters.slippage,
//   );
//
//   const authority = parameters.pool.tokenSwapAuthority();
//
//   return TokenSwap.swapInstruction(
//       parameters.pool.address,
//       authority,
//       parameters.fromAccount.address,
//       poolIntoAccount.address,
//       poolFromAccount.address,
//       parameters.toAccount.address,
//       parameters.pool.poolToken.address,
//       parameters.pool.feeAccount.address,
//       parameters.hostFeePublicKey || null,
//       swapProgramId,
//       TOKEN_PROGRAM_ID,
//       parameters.fromAmount,
//       minimumToAmountWithSlippage.toNumber(),
//   );
// };
//
// export const swap = () => {
//   // Create WSOL or Token account
//   const fromAccount =
//       parameters.fromAccount.mint.address.equals(WRAPPED_SOL_MINT) &&
//       parameters.fromAccount.mint.isSimulated
//           ? await createWrappedSolAccount(
//           parameters.fromAccount,
//           parameters.fromAmount,
//           instructions,
//           cleanupInstructions,
//           signers,
//           )
//           : parameters.fromAccount;
//
//   // get the toAccount from the parameters, or create it if not present
//   const isReverse = isReverseSwap(parameters);
//   const toToken = isReverse ? parameters.pool.tokenA.mint : parameters.pool.tokenB.mint;
//
//   // Token account or Create Token account
//   const toAccount =
//       parameters.toAccount && !parameters.toAccount.mint.address.equals(WRAPPED_SOL_MINT)
//           ? parameters.toAccount
//           : await createAccountByMint(
//           getWallet().pubkey,
//           toToken,
//           instructions,
//           cleanupInstructions,
//           signers,
//           );
//
//   console.log('Executing swap:', parameters);
//
//   const delegate = parameters.pool.tokenSwapAuthority();
//
//   // approveInstruction
//   instructions.push(tokenAPI.approveInstruction(fromAccount, delegate, parameters.fromAmount));
//
//   // swapInstruction
//   instructions.push(
//       createSwapTransactionInstruction({
//         fromAccount,
//         fromAmount: parameters.fromAmount,
//         toAccount,
//         hostFeePublicKey: null,
//         slippage: parameters.slippage || 0,
//         pool: parameters.pool,
//       }),
//   );
// }

type SlippageDirection = 'down' | 'up';
/**
 * Adjust an amount either up or down according to a slippage parameter
 * The default slippage parameter is DEFAULT_SLIPPAGE.
 * e.g. if parameters.slippage is 0.1, the amount is 100, and the direction is
 * "down",
 * the result is 100 * (1 - 0.1) = 90
 *
 * @param amount
 * @param direction
 * @param slippage
 */
export const adjustForSlippage = (
  amount: number,
  direction: SlippageDirection,
  slippage = 0,
) => {
  const slippageFractional = slippage / 100;
  const slippageMultiplier =
    1 + (direction === 'up' ? slippageFractional : -slippageFractional);

  return amount * slippageMultiplier; // TODO: check its need or not ".floor()"
};

const isReverseSwap = (pool: TokenSwapType, fromToken: PublicKey) =>
  pool.mintA.equals(fromToken);

export class Fund {
  // eslint-disable-next-line sonarjs/cognitive-complexity
  static async execute(
    baseTokenAccount: PublicKey,
    baseAmount: Decimal,
    fund: FundType,
    pools: TokenSwapPopulated[],
    slippage = 0.1,
  ) {
    const instructions: TransactionInstruction[] = [];
    const cleanupInstructions: TransactionInstruction[] = [];
    const signers: Account[] = [];

    const {
      poolTokenMint,
      poolTokenMintPopulated,
      assets,
      fundState,
      lqdFeeVault,
      initializerFeeVault,
    } = fund.account.data;

    if (!poolTokenMintPopulated) {
      throw new Error(
        `While Fund execute did not found pool token mint info for fund: ${fund.pubkey.toBase58()}`,
      );
    }

    if (!fundState) {
      throw new Error(`While Fund execute fundState does not exists`);
    }

    const fundVaultAccounts = assets.map((asset) => asset.vaultAddress);

    const [authority] = await PublicKey.findProgramAddress(
      [fund.pubkey.toBuffer()],
      DTF_PROGRAM_ID,
    );

    const userPoolTokenAccount = await createAccountByMint(
      getWallet().pubkey,
      poolTokenMint,
      instructions,
      cleanupInstructions,
      signers,
    );

    const userTransferAuthority = new Account();

    signers.push(userTransferAuthority);

    let amount = new Decimal(0);

    // TODO: decimal.js
    const userAssetsAccountsPromises = assets.map(async (asset, index) => {
      if (!asset.vaultPopulated) {
        throw new Error(
          `While Fund execute did not found vaultPopulated for asset: ${asset.vaultAddress.toBase58()}`,
        );
      }

      const pool = pools.find(
        (pool) =>
          pool.mintA.equals(asset.mint) || pool.mintB.equals(asset.mint),
      );
      if (!pool) {
        throw new Error(
          `While Fund execute did not found pool for asset: ${asset.vaultAddress.toBase58()}`,
        );
      }

      if (!pool.tokenAccountAPopulated || !pool.tokenAccountBPopulated) {
        throw new Error(
          `While Fund execute did not found tokenAccountAPopulated/tokenAccountBPopulated`,
        );
      }

      const weight = fundState.assetWeights[index] / 10;
      const fromAmount = baseAmount.div(weight);

      console.log('fromAmount:', fromAmount.toString());

      /**
       * Swap amount calculation
       */
      const isReverse = isReverseSwap(pool, asset.mint);

      const feeRatio =
        pool.tradeFeeNumerator.toNumber() / pool.tradeFeeDenominator.toNumber();

      // handle slippage by setting a minimum expected TO amount
      // the transaction will fail if the received amount is lower than this.
      const [firstAmountInPool, secondAmountInPool] = isReverse
        ? [
            new Decimal(pool.tokenAccountBPopulated.tokenAmount.amount),
            new Decimal(pool.tokenAccountAPopulated.tokenAmount.amount),
          ]
        : [
            new Decimal(pool.tokenAccountAPopulated.tokenAmount.amount),
            new Decimal(pool.tokenAccountBPopulated.tokenAmount.amount),
          ];

      const adjustedAmount = fromAmount;

      const invariant = firstAmountInPool.mul(secondAmountInPool);
      const newFromAmountInPool = firstAmountInPool.add(adjustedAmount);

      const newToAmountInPool = invariant.divToInt(newFromAmountInPool);
      const grossToAmount = secondAmountInPool.sub(newToAmountInPool);
      const fees = grossToAmount.mul(feeRatio).floor();

      const minimumToAmountWithoutSlippage = grossToAmount.sub(fees).toNumber();

      const minimumToAmountWithSlippage = adjustForSlippage(
        minimumToAmountWithoutSlippage,
        'down',
        slippage,
      );

      /**
       * Swap
       */
      const poolIntoAccount = isReverse
        ? pool.tokenAccountB
        : pool.tokenAccountA;
      const poolFromAccount = isReverse
        ? pool.tokenAccountB
        : pool.tokenAccountA;

      const userAssetAccount = await createAccountByMint(
        getWallet().pubkey,
        poolTokenMint,
        instructions,
        cleanupInstructions,
        signers,
      );

      instructions.push(
        SPLToken.createApproveInstruction(
          TOKEN_PROGRAM_ID,
          baseTokenAccount,
          userTransferAuthority.publicKey,
          getWallet().pubkey,
          [],
          new u64(fromAmount.toNumber()),
        ),
      );

      instructions.push(
        TokenSwap.swapInstruction(
          pool.address,
          new PublicKey(poolTokenMintPopulated.mintAuthority),
          userTransferAuthority.publicKey,
          baseTokenAccount,
          poolIntoAccount,
          poolFromAccount,
          userAssetAccount,
          poolTokenMint,
          pool.feeAccount,
          null,
          SWAP_PROGRAM_ID,
          TOKEN_PROGRAM_ID,
          new Numberu64(fromAmount.toNumber()),
          minimumToAmountWithSlippage,
        ),
      );

      console.log('minimumToAmountWithSlippage:', minimumToAmountWithSlippage);

      /**
       * Pool tokens requested calculation
       */
      const totalPoolTokens = Number(poolTokenMintPopulated.supply);
      const poolAssetQuantity = asset.vaultPopulated.tokenAmount.amount;
      // const poolTokensRequested = new Decimal(
      //   minimumToAmountWithSlippage * poolTokensSupply - poolTokensSupply + 1,
      // ).mul(poolAssetQuantity);
      const poolTokensRequested = new Decimal(
        totalPoolTokens * (minimumToAmountWithSlippage - 1) + 1,
      ).div(poolAssetQuantity);

      amount = amount.add(poolTokensRequested);

      return userTransferAuthority.publicKey;
    });

    const userAssetsAccounts = await Promise.all(userAssetsAccountsPromises);

    // To int
    amount = amount.floor();

    console.log('amount:', amount.toString());

    instructions.push(
      FUND.createExecuteInstruction(
        DTF_PROGRAM_ID,
        fund.pubkey,
        poolTokenMint,
        fundVaultAccounts,
        authority,
        userPoolTokenAccount,
        userAssetsAccounts,
        getWallet().pubkey,
        lqdFeeVault,
        initializerFeeVault,
        HOST_FEE_VAULT,
        TOKEN_PROGRAM_ID,
        new BN(amount.toString()),
      ),
    );

    const transaction = await makeTransaction(
      [...instructions, ...cleanupInstructions],
      signers,
    );

    return sendTransaction(transaction);
  }
}
