import { closeModal, $modals, closeModalFx, openModalFx, openModal } from '.';

const promises = new Map();
let modalId = 0;

closeModalFx.use(({ modalId, result }): void => {
  if (!modalId) {
    throw new Error('Trying to hide modal without modalId');
  }

  const dialogInfo = promises.get(modalId);

  if (dialogInfo) {
    dialogInfo.resolve(result);
    promises.delete(modalId);
  }

  closeModal({ modalId });
});

openModalFx.use(
  ({ modalType, props }): Promise<unknown> => {
    const modals = $modals.getState();

    if (modals.some((modal) => modal.modalType === modalType)) {
      // TODO: custom type of Error
      throw new Error(`Dublicate of modal ${modalType}`);
    }

    modalId = ++modalId;

    openModal({ modalType, modalId, props });

    const promise = new Promise((resolve) => {
      promises.set(modalId, {
        modalId,
        resolve,
      });
    });

    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    promise.modalId = modalId;

    return promise;
  },
);

$modals
  .on(closeModal, (state, payload) =>
    state.filter((dialog) => dialog.modalId !== payload.modalId),
  )
  .on(openModal, (state, payload) => state.concat(payload));
