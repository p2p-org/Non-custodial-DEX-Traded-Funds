import React from 'react';
import ReactDOM from 'react-dom';

import { createInspector } from 'effector-logger/inspector';

import { App } from './App';
import { reportWebVitals } from './reportWebVitals';
import './models/init';
import { css } from '@linaria/core';
import { ress } from './styles/ress';

createInspector();

export const globals = css`
  :global() {
    ${ress}

    html,
    body,
    #root {
      height: 100%;
    }

    body {
      font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI',
        'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans',
        'Helvetica Neue', sans-serif;
      font-feature-settings: 'kern';
      text-rendering: optimizeLegibility;
      -webkit-font-smoothing: antialiased;
    }
  }
`;

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.querySelector('#root'),
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
