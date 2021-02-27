import React from 'react';
import { Router } from 'react-router';
import { useGate } from 'effector-react';

import { AppGate } from 'models/app';
import { history } from 'lib/routing';
import { Pages } from './routes';
import { ModalManager } from './components/common/ModalManager';

export const App: React.FC = () => {
  useGate(AppGate);

  return (
    <>
      <Router history={history}>
        <Pages />
      </Router>
      <ModalManager />
    </>
  );
};
