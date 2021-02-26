import { app } from './index';

app.onCreateEffect((newEffect) => {
  newEffect.failData.watch((payload) => console.error(payload));
});
