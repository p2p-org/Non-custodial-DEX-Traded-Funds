import React from 'react';

import isPropValid from '@emotion/is-prop-valid';
import { CSSProperties } from '@linaria/core';

import { SvgIconType } from 'types/custon';
import arrowTriangle from './assets/arrow-triangle-icon.svg';
import checkmark from './assets/checkmark-icon.svg';

// TODO: check thats wrong with type
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const iconsMap = new Map<string, SvgIconType>([
  ['arrow-triangle', arrowTriangle],
  ['checkmark', checkmark],
]);

export interface IconType {
  name: string;
  size?: string | number;
  height?: string | number;
  width?: string | number;
  style?: CSSProperties;
  className?: string;
}

export const Icon: React.FunctionComponent<IconType> = ({
  name,
  size,
  height,
  width,
  ...props
}) => {
  const validProps: {
    [prop: string]: any;
  } = {};

  Object.keys(props).forEach((prop) => {
    if (isPropValid(prop)) {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      validProps[prop] = props[prop];
    }
  });

  const icon = iconsMap.get(name);

  if (!icon) {
    return null;
  }

  return (
    <svg
      {...validProps}
      viewBox={icon.viewBox}
      height={size || height}
      width={size || width}
    >
      <use xlinkHref={`#${icon.id}`} />
    </svg>
  );
};
