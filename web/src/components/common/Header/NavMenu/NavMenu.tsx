import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { NavLink } from 'react-router-dom';

const Wrapper = styled.div`
  display: flex;
  align-items: center;

  & > :not(:last-child) {
    margin-right: 36px;
  }
`;

const NavMenuLink = styled(NavLink)`
  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 140%;
  opacity: 0.8;
`;

export const NavMenu: FC = () => {
  return (
    <Wrapper>
      <NavMenuLink to="/#myassets">My assets</NavMenuLink>
    </Wrapper>
  );
};
