import { forward } from 'effector';
import { TOKENS } from 'config/tokens';
import { cryptoCompareApiKey } from 'config';
import { AppGate } from '../app';
import { $ratesMap, fetchRatesFx } from './index';

const CRYPTO_COMPARE_API_URL = 'https://min-api.cryptocompare.com/data';

$ratesMap.on(fetchRatesFx.doneData, (state, newRates) => {
  const newState = { ...state };
  newRates.map((rate) => (newState[rate.symbol] = rate.rate));
  return newState;
});

fetchRatesFx.use(async () => {
  const symbols = TOKENS.devnet.map((token) => token.tokenSymbol);

  try {
    const res = await fetch(
      `${CRYPTO_COMPARE_API_URL}/pricemulti?api_key=${cryptoCompareApiKey}&fsyms=${symbols.join(
        ',',
      )}&tsyms=USD`,
    );

    if (!res.ok) {
      throw new Error('Something wrong');
    }

    const result = (await res.json()) as RatesCryptoCompareResponse;

    const rates: FetchRates[] = [];

    symbols.forEach((symbol) => {
      if (result[symbol]) {
        rates.push({ symbol, rate: result[symbol].USD });
      }
    });

    return rates;
  } catch (error) {
    console.error(`Can't get rates for tokens:`, error);
    throw error;
  }
});

// TODO: check it is a right place?
forward({
  from: AppGate.open,
  to: fetchRatesFx,
});
