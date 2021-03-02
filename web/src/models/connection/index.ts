import { PublicKey, PublicKeyAndAccount } from '@solana/web3.js';
import { app } from 'models/app';
import { PoolStatePopulated } from './types';

export const findFundFx = app.createEffect<
  { fundAddress: PublicKey },
  PublicKeyAndAccount<PoolStatePopulated>
>();

export const findFundsFx = app.createEffect<
  void,
  PublicKeyAndAccount<PoolStatePopulated>[]
>();
