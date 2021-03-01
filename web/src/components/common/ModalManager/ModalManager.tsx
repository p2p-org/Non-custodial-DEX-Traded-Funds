/* eslint-disable react/jsx-props-no-spreading */
import React, {
  createRef,
  FC,
  lazy,
  LazyExoticComponent,
  RefObject,
  Suspense,
  useState,
} from 'react';

import { styled } from '@linaria/react';
import { last } from 'ramda';
import { useStore } from 'effector-react';
import { ModalComponentType, ModalState } from 'models/modals/types';
import { $modals, closeModalFx } from 'models/modals';
import { MODAL_INVEST, MODAL_WITHDRAW } from './constants';

const Wrapper = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  z-index: 30;

  width: 100vw;
  height: 100vh;
`;

const ModalContainer = styled.div`
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 1;

  overflow-y: auto;
  overscroll-behavior: none;

  /* Above background */
  &:last-child {
    z-index: 2;
  }
`;

const ModalWrapper = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100%;
  padding: 10px 0;
`;

const ModalBackground = styled.div`
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 2;

  background-color: rgba(0, 0, 0, 0.6);

  user-select: none;
`;

const modalsMap = new Map<string, LazyExoticComponent<any>>([
  [MODAL_INVEST, lazy(() => import('components/modals/Invest'))],
  [MODAL_WITHDRAW, lazy(() => import('components/modals/Withdraw'))],
]);

export const ModalManager: FC = () => {
  const [modalsRefs, setModalsRefs] = useState<{
    [modalId: string]: RefObject<ModalComponentType>;
  }>({});
  const modals = useStore($modals);

  const closeTopModal = async () => {
    const modal = last(modals);
    if (!modal) {
      return;
    }

    const { modalId } = modal;

    const modalRef = modalsRefs[modalId];
    if (modalRef.current?.canClose && !(await modalRef.current.canClose())) {
      return;
    }

    closeModalFx({ modalId });
  };

  const handleWrapperClick = (e: React.MouseEvent<HTMLDivElement>) => {
    // handle click only on element
    if (e.target !== e.currentTarget) {
      return;
    }

    closeTopModal();
  };

  const getReadyDialogs = () => {
    const dialogs: (ModalState & {
      ModalComponent: LazyExoticComponent<any>;
    })[] = [];

    for (const { modalType, modalId, props } of modals) {
      const ModalComponent = modalsMap.get(modalType);
      if (ModalComponent) {
        dialogs.push({
          modalType,
          modalId,
          props,
          ModalComponent,
        });
      }
    }

    return dialogs;
  };

  const dialogsInfo = getReadyDialogs();

  const dialogs = dialogsInfo.map(({ modalId, props, ModalComponent }) => {
    let modalRef = modalsRefs[modalId];

    if (!modalRef) {
      modalRef = createRef();
      setModalsRefs((state) => ({
        ...state,
        [modalId]: modalRef,
      }));
    }

    return (
      <Suspense fallback={null} key={modalId}>
        <ModalContainer>
          <ModalWrapper onClick={handleWrapperClick}>
            <ModalComponent
              {...props}
              modalId={modalId}
              modalRef={modalRef}
              close={(result: any) => closeModalFx({ modalId, result })}
            />
          </ModalWrapper>
        </ModalContainer>
      </Suspense>
    );
  });

  if (dialogs.length > 0) {
    return (
      <Wrapper>
        <ModalBackground />
        {dialogs}
      </Wrapper>
    );
  }

  return null;
};
