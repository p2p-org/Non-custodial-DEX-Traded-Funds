import { PublicKey, PublicKeyAndAccount } from '@solana/web3.js';
import { app } from 'models/app';
import { TokenSwapType } from './layouts/tokenSwap';
import { FundType } from './types';

/**
 * Stores
 */
export const $funds = app.createStore<FundType[]>([]);

/**
 * Effects
 */
export const findFundFx = app.createEffect<PublicKey, FundType>();

export const findFundsFx = app.createEffect<void, FundType[]>();
export const findPoolsFx = app.createEffect<void, TokenSwapType[]>();
