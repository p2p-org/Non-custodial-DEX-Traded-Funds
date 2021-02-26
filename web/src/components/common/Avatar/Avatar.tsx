import React, { FC } from 'react';

import { styled } from '@linaria/react';

const Wrapper = styled.div`
  width: 56px;
  height: 56px;

  background: azure;
  border-radius: 4px;
`;

interface Props {}

export const Avatar: FC<Props> = (props) => {
  return <Wrapper />;
};
