import { PublicKeyAndAccount } from '@solana/web3.js';
import { app } from 'models/app';
import { PoolState } from '../../../../js/lib/fund';

export const findFundFx = app.createEffect<
  void,
  PublicKeyAndAccount<PoolState>
>();

export const findFundsFx = app.createEffect<
  void,
  PublicKeyAndAccount<PoolState>[]
>();
