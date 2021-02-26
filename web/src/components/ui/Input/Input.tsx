import React, { FC, InputHTMLAttributes } from 'react';

import { styled } from '@linaria/react';

const WrapperLabel = styled.label`
  display: flex;
  align-items: center;

  height: 48px;

  background: #ffffff;
  border: 1px solid rgba(163, 165, 186, 0.5);
  box-sizing: border-box;
  border-radius: 4px;
`;

const InputElement = styled.input`
  height: 100%;
  width: 100%;
  padding: 0 20px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-size: 16px;
  line-height: 100%;

  outline: 0;

  &::placeholder {
    color: #8c8e99;
  }
`;

export const Input: FC<InputHTMLAttributes<HTMLInputElement>> = ({
  value,
  style,
  className,
}) => {
  return (
    <WrapperLabel style={style} className={className}>
      <InputElement value={value} />
    </WrapperLabel>
  );
};
