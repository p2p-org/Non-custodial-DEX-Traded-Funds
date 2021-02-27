import { ComponentType } from 'react';

export interface ModalState {
  modalType: string;
  modalId: number;
  props: any;
}

export type ModalsState = ModalState[];

export type ModalComponentType<P = unknown> = ComponentType<P> & {
  canClose(): Promise<any>;
};
