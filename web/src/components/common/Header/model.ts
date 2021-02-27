import { forward } from 'effector';
import { ButtonClick } from 'types/effector';
import { app } from 'models/app';
import { connectFx } from 'models/wallet';

export const connectClicked = app.createEvent<ButtonClick>();

forward({
  from: connectClicked,
  to: connectFx,
});
