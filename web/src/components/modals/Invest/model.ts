import { forward } from 'effector';
import { createGate } from 'effector-react';
import { PublicKey } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { DTF_PROGRAM_ID } from 'config';
import { app } from 'models/app';
import { findPoolsFx } from 'models/connection';
import { TokenSwapType } from 'models/connection/layouts/tokenSwap';
import { ButtonClick } from 'types/effector';
import { FundType } from 'models/connection/types';
import { Fund } from '../../../../../js/client/fund';

export const InvestGate = createGate<PublicKey>();

export const $pools = app.createStore<TokenSwapType[]>([]);
export const $currentFund = app.createStore<FundType | null>(null);

export const investClicked = app.createEvent<ButtonClick>();

investClicked.watch(async () => {
  const fund = $currentFund.getState();
  if (!fund) {
    console.error('fund did not passed');
    return;
  }

  console.log(111, fund);

  const { poolTokenMint, lqdFeeVault, initializerFeeVault } = fund.account.data;
  const fundVaultAccounts = [];

  const [authority] = await PublicKey.findProgramAddress(
    [fund.pubkey.toBuffer()],
    DTF_PROGRAM_ID,
  );

  const userPoolTokenAccount = '';
  const userAssetsAccounts = [];
  const authorityUserAccounts = [];
  const refferFeeVault = '';
  const amount = 0;

  // Fund.createExecuteInstruction(
  //   DTF_PROGRAM_ID,
  //   fund.pubkey,
  //   poolTokenMint,
  //   fundVaultAccounts,
  //   authority,
  //   userPoolTokenAccount,
  //   userAssetsAccounts,
  //   authorityUserAccounts,
  //   lqdFeeVault,
  //   initializerFeeVault,
  //   refferFeeVault,
  //   TOKEN_PROGRAM_ID,
  //   amount,
  // );
});

$pools.on(findPoolsFx.doneData, (state, pools) => [...state, ...pools]);

forward({
  from: InvestGate.open,
  to: findPoolsFx,
});

forward({
  from: InvestGate.open,
  to: $currentFund,
});
