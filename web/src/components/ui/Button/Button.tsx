import React, { ButtonHTMLAttributes, FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';

const ButtonStyled = styled.button`
  display: flex;
  align-items: center;

  height: 48px;
  padding: 0 40px;

  font-family: TT Firs Neue, sans-serif;
  font-weight: 500;
  font-size: 16px;
  line-height: 100%;

  border-radius: 4px;

  &.primary {
    color: #fff;

    background: #62cf7a;
  }

  &.hollow {
    color: #5887ff;

    background: #ffffff;
  }
`;

interface Props {
  primary?: boolean;
  hollow?: boolean;
}

export const Button: FC<Props & ButtonHTMLAttributes<HTMLButtonElement>> = ({
  onClick,
  primary,
  hollow,
  children,
  style,
  className,
}) => {
  return (
    <ButtonStyled
      onClick={onClick}
      style={style}
      className={classNames(className, { primary, hollow })}
    >
      {children}
    </ButtonStyled>
  );
};
