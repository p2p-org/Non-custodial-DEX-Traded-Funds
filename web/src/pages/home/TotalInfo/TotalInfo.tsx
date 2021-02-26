import React, { FC, HTMLAttributes } from 'react';

import { styled } from '@linaria/react';

const Wrapper = styled.div`
  display: grid;
  grid-auto-flow: column;
  grid-gap: 88px;
`;

const Description = styled.div`
  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-size: 16px;
  line-height: 140%;
`;

const StatsWrapper = styled.div`
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: 184px;
  grid-gap: 20px;

  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
`;

const Stats = styled.div``;

const StatsTitle = styled.div`
  margin-bottom: 10px;

  color: #8c8e99;
  font-family: Titillium Web, sans-serif;
  font-size: 14px;
  line-height: 140%;
`;

const StatsValue = styled.div`
  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 24px;
  line-height: 140%;
`;

interface Props {}

export const TotalInfo: FC<Props & HTMLAttributes<HTMLDivElement>> = ({
  style,
  className,
}) => {
  return (
    <Wrapper style={style} className={className}>
      <Description>
        Capital under our management has shown double growth with the market
        falling by 80%. We offer complete. ‘one-stop’ DeFi solution and are a
        preferred lender to leading private equity investments
      </Description>
      <StatsWrapper>
        <Stats>
          <StatsTitle>Funds (Total created)</StatsTitle>
          <StatsValue>1 200</StatsValue>
        </Stats>
        <Stats>
          <StatsTitle>Investments (DeFi to $)</StatsTitle>
          <StatsValue>2,34 M</StatsValue>
        </Stats>
        <Stats>
          <StatsTitle>Average profit (per month)</StatsTitle>
          <StatsValue>1 200</StatsValue>
        </Stats>
      </StatsWrapper>
    </Wrapper>
  );
};
