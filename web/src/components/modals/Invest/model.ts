import { combine, forward } from 'effector';
import { createGate } from 'effector-react';
import Decimal from 'decimal.js';
import { any, or, pathEq } from 'ramda';
import { app } from 'models/app';
import { ButtonClick, InputChange } from 'types/effector';
import { AssetType, FundType } from 'models/types';
import { $pools, findPoolsFx } from 'models/pools';
import { Fund } from 'api/fund';
import {
  $tokenAccounts,
  getParsedTokenAccountsByOwnerFx,
} from 'models/connection';
import { TokenSwapType } from 'models/connection/layouts/tokenSwap';
import { fetchRatesFx } from '../../../models/rates';
import { findFundFx, findFundsFx } from '../../../models/funds';

export const InvestGate = createGate<FundType>();

export const $amount = app.createStore<Decimal>(new Decimal(0));

export const $currentFund = app.createStore<FundType | null>(null);

export const investClicked = app.createEvent<ButtonClick>();

export const setAmount = app.createEvent<Decimal>();
export const changeAmount = app.createEvent<InputChange>();

export const $baseTokenAccount = combine(
  $tokenAccounts,
  $currentFund,
  (tokenAccounts, currentFund) => {
    if (!currentFund) {
      return null;
    }

    const token = currentFund.account.data.fundState?.basicAsset.mint.toBase58();

    if (!token) {
      return null;
    }

    const filteredTokenAccounts = tokenAccounts.filter(
      (tokenAccount) => tokenAccount.account.data.mint === token,
    );

    if (filteredTokenAccounts.length === 0) {
      return null;
    }

    return filteredTokenAccounts[0];
  },
);

export const $isLoading = combine(
  getParsedTokenAccountsByOwnerFx.pending,
  fetchRatesFx.pending,
  findFundsFx.pending,
  findFundFx.pending,
  findPoolsFx.pending,
  (a, b, c, d, e) => a || b || c || d || e,
);

$amount
  .on(setAmount, (_, newAmount) => newAmount)
  .on(changeAmount, (_, e) => new Decimal(Number(e.currentTarget.value)));

const isPoolAsset = (assets: AssetType[]) => (pool: TokenSwapType) =>
  any(or(pathEq(['mint'], pool.mintA), pathEq(['mint'], pool.mintB)), assets);

investClicked.watch(async () => {
  const fund = $currentFund.getState();
  if (!fund) {
    console.error('fund did not passed');
    return;
  }

  const baseTokenAccount = $baseTokenAccount.getState();
  if (!baseTokenAccount) {
    console.error('baseTokenAccount did not passed');
    return;
  }

  const assetsPools = $pools
    .getState()
    .filter(isPoolAsset(fund.account.data.assets));
  if (assetsPools.length !== fund.account.data.assets.length) {
    console.error('did not enought pools for assets');
    return;
  }

  const amount = $amount
    .getState()
    .mul(10 ** baseTokenAccount.account.data.tokenAmount.decimals);

  Fund.execute(baseTokenAccount.pubkey, amount, fund, assetsPools);
});

forward({
  from: InvestGate.open,
  to: $currentFund,
});
