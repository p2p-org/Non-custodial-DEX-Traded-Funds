import React, { ButtonHTMLAttributes, FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';

const ButtonElement = styled.button`
  height: 48px;
  padding: 0 40px;

  font-family: TT Firs Neue, sans-serif;
  font-weight: 500;
  font-size: 16px;
  line-height: 100%;
  white-space: nowrap;

  border-radius: 4px;
  outline: 0;

  &.primary {
    color: #fff;

    background: #62cf7a;
  }

  &.hollow {
    color: #5887ff;

    background: #ffffff;
  }

  &:disabled {
    color: #fff;

    background: #a3a5ba;
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
  disabled,
  children,
  style,
  className,
}) => {
  return (
    <ButtonElement
      disabled={disabled}
      onClick={onClick}
      style={style}
      className={classNames(className, { primary, hollow })}
    >
      {children}
    </ButtonElement>
  );
};
