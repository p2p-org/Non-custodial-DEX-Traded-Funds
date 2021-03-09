import { $tokenAccounts, getParsedTokenAccountsByOwnerFx } from './index';

import './effects';

$tokenAccounts.on(
  getParsedTokenAccountsByOwnerFx.doneData,
  (_, tokenAccounts) => {
    if (tokenAccounts.length === 0) {
      return [];
    }

    console.log(333, tokenAccounts);

    return tokenAccounts.sort(
      (a, b) =>
        b.account.data.tokenAmount.uiAmount -
        a.account.data.tokenAmount.uiAmount,
    );
  },
);
