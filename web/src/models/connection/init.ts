/* eslint-disable import/export */
import { PublicKey } from '@solana/web3.js';
import { AppGate } from 'models/app';
import { $funds, findFundFx } from './index';

export * from './effects/findFundFx';
export * from './effects/findFundsFx';
export * from './effects/findPoolsFx';

$funds.on(findFundFx.doneData, (state, newFund) => [
  ...state.filter((find) => !find.pubkey.equals(newFund.pubkey)),
  newFund,
]);

// TODO: temp
const fundAddress = new PublicKey(
  't72redTRJkPtUmTWWvPyjnkFKGVrHakv3DBQTheY4oD',
);
AppGate.open.watch(() => {
  findFundFx(fundAddress);
});
