import * as WalletAPI from 'api/wallet';
import { WalletEvent } from 'api/wallet/Wallet';

import {
  $cluster,
  $connected,
  $type,
  $wallet,
  connectFx,
  disconnectFx,
} from '.';

connectFx.use(async () => {
  const cluster = $cluster.getState();
  const type = $type.getState();
  const wallet = await WalletAPI.connect(cluster, type);

  wallet.on(WalletEvent.DISCONNECT, () => {
    disconnectFx();
    console.error('Wallet disconnected');
  });

  wallet.on(WalletEvent.CONFIRMED, ({ transactionSignature }) =>
    console.info(`Confirmed: ${transactionSignature}`),
  );

  console.info('Wallet connected');

  return wallet.pubkey.toBase58();
});

disconnectFx.use(() => {
  WalletAPI.disconnect();
  console.error('Wallet disconnected');
});

$connected.on(connectFx.done, () => true).on(disconnectFx, () => false);

$wallet.on(connectFx.doneData, (_, wallet) => wallet);
