import { Cluster } from '@solana/web3.js';
import { WalletType } from 'api/wallet';
import { app } from '../app';

export const connectFx = app.createEffect<void, string>();

export const disconnectFx = app.createEffect<void, void>();

export const $cluster = app.createStore<Cluster>('devnet');

export const clusterPersisted = app.createEvent<any>();

export const $type = app.createStore<WalletType>(WalletType.SOLLET);

export const $connected = app.createStore<boolean>(false);

export const $wallet = app.createStore<string | null>(null);
