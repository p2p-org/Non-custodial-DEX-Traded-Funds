import React, { FC, HTMLAttributes } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';
import { FundRow } from './FundRow';
import { Column } from './common/Column';

const Wrapper = styled.div``;

const Title = styled.div`
  margin-bottom: 38px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 24px;
  line-height: 100%;
`;

const ColumnsHeader = styled.div`
  display: flex;
  align-items: center;

  height: 36px;
  padding: 0 26px;

  background: #f6f6f8;
  border-radius: 12px;
`;

const ColumnName = styled.div`
  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 13px;
  line-height: 140%;

  cursor: pointer;

  &:hover {
    color: #000000;
  }
`;

const FundList = styled.div`
  display: grid;
  grid-auto-flow: row;
  grid-gap: 24px;

  margin-top: 20px;
`;

interface Props {}

export const Funds: FC<Props & HTMLAttributes<HTMLDivElement>> = ({
  style,
  className,
}) => {
  return (
    <Wrapper style={style} className={className}>
      <Title>DTFs</Title>
      <ColumnsHeader>
        <Column className={classNames({ name: true })}>
          <ColumnName>Name</ColumnName>
        </Column>
        <Column className={classNames({ marketCap: true })}>
          <ColumnName>Market Cap</ColumnName>
        </Column>
        <Column className={classNames({ price: true })}>
          <ColumnName>Price</ColumnName>
        </Column>
        <Column className={classNames({ since: true })}>
          <ColumnName>Since inception</ColumnName>
        </Column>
        <Column className={classNames({ balance: true })}>
          <ColumnName>Balance</ColumnName>
        </Column>
      </ColumnsHeader>
      <FundList>
        <FundRow />
        <FundRow />
        <FundRow />
        <FundRow />
        <FundRow />
        <FundRow />
      </FundList>
    </Wrapper>
  );
};
