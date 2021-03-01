import { app } from 'models/app';

export const $ratesMap = app.createStore<Rates>({});

export const fetchRatesFx = app.createEffect<void, FetchRates[]>();
