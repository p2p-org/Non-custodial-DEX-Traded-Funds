import React, { useMemo } from 'react';
import { Redirect } from 'react-router-dom';
import { renderRoutes, RouteConfig } from 'react-router-config';

import { useStore } from 'effector-react';

import { Home } from 'pages/home';
import { NotFound } from 'pages/notFound';
import { $connected } from 'models/wallet';
import { Fund } from 'pages/fund';

type RouteConfigCustom = RouteConfig & { needAuth?: boolean };

export const makeRoutes = (isAuth: boolean): RouteConfigCustom[] =>
  ([
    {
      path: '/',
      exact: true,
      component: Home,
    },
    {
      path: '/:fund',
      exact: true,
      component: Fund,
    },
    {
      path: '*',
      component: NotFound,
    },
  ] as RouteConfigCustom[]).map((route) => {
    if (route.needAuth && route.needAuth !== isAuth) {
      route.component = () => <Redirect to="/" />;
      return route;
    }

    return route;
  });

export const Pages = () => {
  // TODO: Cannot update a component (`Pages`) while rendering a different component
  const connected = useStore($connected);

  return useMemo(() => renderRoutes(makeRoutes(connected)), [connected]);
};
