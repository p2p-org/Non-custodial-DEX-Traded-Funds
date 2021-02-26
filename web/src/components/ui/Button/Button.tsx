import React, { ButtonHTMLAttributes, FC } from 'react';

import { styled } from '@linaria/react';

const ButtonStyled = styled.button``;

export const Button: FC<ButtonHTMLAttributes<HTMLButtonElement>> = ({
  children,
  ...props
}) => {
  return (
    // eslint-disable-next-line react/jsx-props-no-spreading
    <ButtonStyled {...props}>{children}</ButtonStyled>
  );
};
