import React, { FC, HTMLAttributes } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';
import { Link } from 'react-router-dom';
import { FundRow } from './FundRow';

const Wrapper = styled.div``;

const Header = styled.div`
  display: flex;
  justify-content: space-between;

  margin-bottom: 16px;
`;

const Left = styled.div`
  display: flex;
  align-items: center;
`;

const Title = styled.h3`
  color: #000000;
  font-family: TT Firs Neue, sans-serif;
  font-size: 24px;
  line-height: 100%;
`;

const CreateLink = styled(Link)`
  margin-left: 24px;

  color: #5887ff;
  font-family: TT Firs Neue, sans-serif;
  font-size: 16px;
  line-height: 100%;
`;

const Right = styled.div`
  display: grid;
  grid-auto-flow: column;
  grid-gap: 10px;
`;

const SortButton = styled.button`
  height: 52px;
  padding: 0 20px;

  color: #8c8e99;
  font-family: Titillium Web, sans-serif;
  font-size: 14px;
  line-height: 140%;

  background: #fff;
  border-radius: 4px;

  &.active {
    color: #000000;

    background: #f6f6f8;
  }
`;

const Content = styled.div`
  display: grid;
  grid-auto-flow: row;
  grid-gap: 24px;
`;

interface Props {}

export const Funds: FC<Props & HTMLAttributes<HTMLDivElement>> = ({
  style,
  className,
}) => {
  return (
    <Wrapper style={style} className={className}>
      <Header>
        <Left>
          <Title>Funds</Title>
          <CreateLink to="/#create">+ Create new</CreateLink>
        </Left>
        <Right>
          <SortButton className={classNames({ active: true })}>
            Sorting 1
          </SortButton>
          <SortButton>Sorting 2</SortButton>
          <SortButton>Sorting 3</SortButton>
          <SortButton>Sorting 4</SortButton>
        </Right>
      </Header>
      <Content>
        <FundRow />
        <FundRow />
      </Content>
    </Wrapper>
  );
};
