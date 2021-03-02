interface RatesCryptoCompareResponse {
  [market: string]: {
    [currency: string]: number;
  };
}

interface Rates {
  [currency: string]: number;
}

interface FetchRates {
  symbol: string;
  rate: number;
}
