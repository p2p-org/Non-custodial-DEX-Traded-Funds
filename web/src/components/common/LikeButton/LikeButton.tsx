import React, { FC } from 'react';

import { styled } from '@linaria/react';
import likeImg from './like.png';

const WrapperButton = styled.button`
  display: flex;
  align-items: center;

  height: 48px;
  padding: 0 20px;

  color: #000000;
  font-family: TT Firs Neue, sans-serif;
  font-weight: 500;
  font-size: 16px;
  line-height: 100%;

  background: #fbfbfd;
  border-radius: 4px;
`;

const Like = styled.div`
  width: 24px;
  height: 24px;
  margin: -4px 10px 0 0;

  background: url('${likeImg}') no-repeat 50% 50%;
  background-size: 24px;
`;

interface Props {}

export const LikeButton: FC<Props> = (props) => {
  return (
    <WrapperButton>
      <Like /> 10
    </WrapperButton>
  );
};
