import React from 'react';
import ReactDOM from 'react-dom';

import { createInspector } from 'effector-logger/inspector';

import { App } from './App';
import { reportWebVitals } from './reportWebVitals';
import './models/init';
import { css } from '@linaria/core';
import { ress } from './styles/ress';
import { fonts } from 'styles/fonts';

createInspector();

export const globals = css`
  :global() {
    ${ress}
    ${fonts}
    html,
    body,
    #root {
      height: 100%;
    }

    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto',
        'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans',
        'Helvetica Neue', sans-serif;
      font-feature-settings: 'kern';
      text-rendering: optimizeLegibility;
      -webkit-font-smoothing: antialiased;

      background: #fbfbfd;
    }

    #root {
      background-size: 100% 616px;
      background: linear-gradient(180deg, #f0f1f5 0%, #fbfbfd 100%) no-repeat;
    }

    a,
    a:focus,
    a:active {
      text-decoration: none;
      color: inherit;
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
