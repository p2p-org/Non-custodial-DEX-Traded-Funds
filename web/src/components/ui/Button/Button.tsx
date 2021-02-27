import React, { ButtonHTMLAttributes, FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';

const ButtonElement = styled.button`
  height: 46px;
  padding: 0 40px;

  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 14px;
  line-height: 100%;
  white-space: nowrap;

  border-radius: 12px;
  outline: 0;

  &.primary {
    color: #fff;

    background: #5887ff;
  }

  &.hollow {
    color: #5887ff;

    background: #ffffff;
  }

  &.full {
    width: 100%;
  }

  &:disabled {
    color: #fff;

    background: #a3a5ba;
  }
`;

interface Props {
  primary?: boolean;
  hollow?: boolean;
  full?: boolean;
}

export const Button: FC<Props & ButtonHTMLAttributes<HTMLButtonElement>> = ({
  onClick,
  primary,
  hollow,
  full,
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
      className={classNames(className, { primary, hollow, full })}
    >
      {children}
    </ButtonElement>
  );
};
