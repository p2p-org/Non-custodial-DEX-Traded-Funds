import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { Link } from 'react-router-dom';
import { Avatar } from '../Avatar';

const Wrapper = styled.div`
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

interface Props {}

export const FundInfo: FC<Props> = (props) => {
  return (
    <Wrapper>
      <Avatar />
      <InfoWrapper>
        <FundName>SPFG SOL Prudential Fund — Growth</FundName>
        <FundDetails>
          <FundType>DeFi Tokens</FundType>
          <FundDelimiter>|</FundDelimiter>
          <FundStrategy>Long strategy</FundStrategy>
        </FundDetails>
      </InfoWrapper>
    </Wrapper>
  );
};
