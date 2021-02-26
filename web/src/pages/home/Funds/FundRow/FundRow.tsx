import React, { FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';
import { Button } from 'components/ui/Button';
import { Avatar } from 'components/common/Avatar';

import likeImg from './like.png';

const Wrapper = styled.div`
  background: #ffffff;
  border-radius: 4px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.05);
`;

const HeaderWrapper = styled.div`
  display: flex;
  justify-content: space-between;

  padding: 24px;
`;

const HeaderLeft = styled.div`
  display: flex;
  align-items: center;
`;

const InfoWrapper = styled.div`
  margin-left: 20px;
`;

const FundName = styled.h4`
  margin-bottom: 4px;

  color: #000000;
  font-family: TT Firs Neue, sans-serif;
  font-weight: 500;
  font-size: 18px;
  line-height: 140%;
`;

const FundDetails = styled.div`
  display: flex;

  color: #8c8e99;
  font-family: Titillium Web, sans-serif;
  font-size: 14px;
  line-height: 140%;
`;

const FundType = styled.div``;

const FundDelimiter = styled.div`
  margin: 0 5px 0 10px;
`;

const FundStrategy = styled.div`
  color: #5887ff;
`;

const BottomWrapper = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;

  padding: 16px 24px;

  border-top: 1px solid rgba(163, 165, 186, 0.2);
`;

const StatsWrapper = styled.div`
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: 184px;
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

const LikeButton = styled.button`
  display: flex;
  align-items: center;

  height: 48px;
  padding: 0 20px;

  color: #000000;
  font-family: TT Firs Neue, sans-serif;
  font-size: 16px;
  line-height: 100%;

  background: #fbfbfd;
  border-radius: 4px;
`;

const Like = styled.div`
  width: 24px;
  height: 24px;
  margin: -4px 10px 0 0;

  background: url('${likeImg}') no-repeat 50% 50%;
  background-size: 24px;
`;

interface Props {}

export const FundRow: FC<Props> = (props) => {
  return (
    <Wrapper>
      <HeaderWrapper>
        <HeaderLeft>
          <Avatar />
          <InfoWrapper>
            <FundName>SPFG SOL Prudential Fund — Growth</FundName>
            <FundDetails>
              <FundType>DeFi Tokens</FundType>
              <FundDelimiter>|</FundDelimiter>
              <FundStrategy>Long strategy</FundStrategy>
            </FundDetails>
          </InfoWrapper>
        </HeaderLeft>
        <Button primary>+ Invest now</Button>
      </HeaderWrapper>
      <BottomWrapper>
        <StatsWrapper>
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
        </StatsWrapper>
        <LikeButton>
          <Like /> 10
        </LikeButton>
      </BottomWrapper>
    </Wrapper>
  );
};
