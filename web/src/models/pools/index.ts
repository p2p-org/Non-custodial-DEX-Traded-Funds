import { app } from 'models/app';
import { TokenSwapPopulated } from 'models/types';

export const $pools = app.createStore<TokenSwapPopulated[]>([]);

export const findPoolsFx = app.createEffect<void, TokenSwapPopulated[]>();
