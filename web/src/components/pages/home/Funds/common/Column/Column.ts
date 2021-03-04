import { styled } from '@linaria/react';

export const Column = styled.div`
  display: flex;

  width: 100%;

  &.name {
    max-width: 263px;
  }

  &.marketCap {
    max-width: 169px;
  }

  &.price {
    max-width: 123px;
  }

  &.since {
    max-width: 154px;
  }

  &.balance {
    max-width: 121px;
  }
`;
