import React, { FC, HTMLAttributes } from 'react';

import { styled } from '@linaria/react';

const Wrapper = styled.div`
  width: 100%;
  max-width: 1068px;
  margin: 0 auto;
`;

export const MainContainer: FC<HTMLAttributes<HTMLDivElement>> = ({
  children,
  style,
  className,
}) => {
  return (
    <Wrapper style={style} className={className}>
      {children}
    </Wrapper>
  );
};
