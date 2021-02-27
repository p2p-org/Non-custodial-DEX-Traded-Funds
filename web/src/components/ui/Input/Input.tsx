import React, { FC, InputHTMLAttributes, useState } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';

const Somefix = styled.div`
  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 100%;
`;

const WrapperLabel = styled.label`
  display: flex;
  align-items: center;
  justify-content: space-between;

  height: 56px;
  padding: 0 20px;

  background: #fbfbfd;
  border-radius: 12px;

  &.isFocus {
    background: #fbfbfd;
    border: 1px solid #5887ff;

    ${Somefix} {
      color: #000000;
    }
  }
`;

const InputElement = styled.input`
  height: 100%;
  width: auto;
  padding: 0 20px;

  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 24px;
  line-height: 100%;
  text-align: center;

  outline: 0;

  &::placeholder {
    color: #a3a5ba;
  }

  &:focus {
    color: #5887ff;
  }
`;

interface Props {
  prefix: string;
  postfix: string;
}

export const Input: FC<Props & InputHTMLAttributes<HTMLInputElement>> = ({
  prefix,
  value,
  postfix,
  style,
  className,
}) => {
  const [isFocus, setIsFocus] = useState(false);

  const handleFocus = () => {
    setIsFocus(true);
  };

  const handleBlur = () => {
    setIsFocus(false);
  };

  return (
    <WrapperLabel style={style} className={classNames(className, { isFocus })}>
      <Somefix>{prefix}</Somefix>
      <InputElement value={value} onFocus={handleFocus} onBlur={handleBlur} />
      <Somefix>{postfix}</Somefix>
    </WrapperLabel>
  );
};
