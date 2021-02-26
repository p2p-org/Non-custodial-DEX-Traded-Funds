import React, { FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';

const Wrapper = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fill, 184px);
  grid-gap: 20px;
`;

const Stats = styled.div``;

const StatsTitle = styled.div`
  margin-bottom: 4px;

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

  &.alert {
    color: #ffa631;
  }

  &.profit {
    color: #62cf7a;
  }
`;

interface Props {
  isFundRow?: boolean;
}

export const FundStats: FC<Props> = ({ isFundRow }) => {
  return (
    <Wrapper>
      {isFundRow ? (
        <>
          <Stats>
            <StatsTitle>Fund size</StatsTitle>
            <StatsValue>1.2M SOL</StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>APY</StatsTitle>
            <StatsValue className={classNames({ alert: true })}>
              - 3,29%
            </StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>Index points</StatsTitle>
            <StatsValue>8.9 pt</StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>1 Tk Price</StatsTitle>
            <StatsValue>15 USDC</StatsValue>
          </Stats>
        </>
      ) : (
        <>
          <Stats>
            <StatsTitle>Fund size</StatsTitle>
            <StatsValue>1.2M SOL</StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>Return (Average)</StatsTitle>
            <StatsValue className={classNames({ alert: true })}>
              - 3,29%
            </StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>Volume</StatsTitle>
            <StatsValue>13 480 tk</StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>Index points</StatsTitle>
            <StatsValue>8.9 pt</StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>1 Tk Price</StatsTitle>
            <StatsValue>15 USDC</StatsValue>
          </Stats>
          <Stats>
            <StatsTitle>Data</StatsTitle>
            <StatsValue>Null</StatsValue>
          </Stats>
        </>
      )}
    </Wrapper>
  );
};
