import { app } from 'models/app';
import { TokenAccountType } from '../types';

export const $tokenAccounts = app.createStore<TokenAccountType[]>([]);

export const getParsedTokenAccountsByOwnerFx = app.createEffect<
  void,
  TokenAccountType[]
>();
