import React, { FC } from 'react';

import { styled } from '@linaria/react';
import avatarImg from './avatar.png';

const Wrapper = styled.div<{
  src?: string;
  size?: string | number;
  circle?: boolean;
}>`
  width: ${(props) => props.size || '56'}px;
  height: ${(props) => props.size || '56'}px;

  background-color: #f6f6f8;
  background-image: ${(props) => (props.src ? `url("${props.src}")` : 'none')};
  background-size: ${(props) => props.size || '56'}px;
  background-repeat: no-repeat;
  background-position: center;
  border-radius: ${(props) => (props.circle ? '50%' : '4px')};

  &:not([src]) {
    background-image: url('${avatarImg}');
    background-size: 34px 31px;
  }
`;

interface Props {
  // TODO: src is temp, use symbol and find icon
  src?: string;
  symbol?: string;
  size?: string | number;
  circle?: boolean;
}

export const Avatar: FC<Props> = ({ src, symbol, size, circle }) => {
  return <Wrapper src={src} size={size} circle={circle} />;
};
