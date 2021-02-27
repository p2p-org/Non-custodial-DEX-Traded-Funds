import { app } from 'models/app';
import { ModalsState, ModalState } from './types';

export const $modals = app.createStore<ModalsState>([]);

export const closeModalFx = app.createEffect<
  {
    modalId: number;
    result?: any;
  },
  void
>();

export const closeModal = app.createEvent<{ modalId: number }>();

export const openModalFx = app.createEffect<
  {
    modalType: string;
    props?: any;
  },
  Promise<unknown>
>();

export const openModal = app.createEvent<ModalState>();
