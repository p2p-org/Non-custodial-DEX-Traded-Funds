import { forward } from 'effector';
import { createGate } from 'effector-react';
import { PublicKey, PublicKeyAndAccount } from '@solana/web3.js';
import { app } from 'models/app';
import { findFundFx } from 'models/connection';
import { PoolStatePopulated } from 'models/connection/types';

export const FundsGate = createGate<{ fundAddress: PublicKey }>();

export const $funds = app.createStore<
  PublicKeyAndAccount<PoolStatePopulated>[]
>([]);

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
