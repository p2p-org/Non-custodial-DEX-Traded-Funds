import { PublicKey } from '@solana/web3.js';
import { app } from '../app';
import { FundType } from '../types';

/**
 * Stores
 */
export const $funds = app.createStore<FundType[]>([]);

/**
 * Effects
 */
export const findFundFx = app.createEffect<PublicKey, FundType>();

export const findFundsFx = app.createEffect<void, FundType[]>();
