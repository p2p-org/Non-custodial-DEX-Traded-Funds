import { forward } from 'effector';
import { createGate } from 'effector-react';
import { PublicKeyAndAccount } from '@solana/web3.js';
import { app } from 'models/app';
import { findFundFx } from 'models/connection';
import { PoolState } from '../../../../../js/lib/fund';

export const FundsGate = createGate();

export const $funds = app.createStore<PublicKeyAndAccount<PoolState>[]>([]);

// $funds.on(findFundsFx.doneData, (_, funds) => funds);

$funds.on(findFundFx.doneData, (state, newFund) => [
  ...state.filter((find) => !find.pubkey.equals(newFund.pubkey)),
  newFund,
]);

forward({
  from: FundsGate.open,
  to: findFundFx,
  // to: findFundsFx,
});
