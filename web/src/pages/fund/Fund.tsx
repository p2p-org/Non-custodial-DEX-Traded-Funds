import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { TOKENS } from '@solana/spl-token-registry';
import { MainContainer } from 'components/common/MainContainer';
import { Header } from 'components/common/Header';
import { FundStats } from 'components/common/FundStats';
import { Avatar } from 'components/common/Avatar';
import { FundInfo } from '../../components/common/FundInfo';
import { LikeButton } from '../../components/common/LikeButton';
import { Invest } from './Invest';
import linkImg from './link.png';

const Wrapper = styled.div``;

const Container = styled(MainContainer)`
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: 592px 456px;
  grid-gap: 20px;

  margin: 34px auto;
`;

const Column = styled.div``;

const BorderBottomWrapper = styled.div`
  & > :not(:last-child) {
    border-bottom: 1px solid rgba(163, 165, 186, 0.2);
  }
`;

const TopWrapper = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;

  padding: 24px 0;

  border-bottom: 1px solid rgba(0, 0, 0, 0.1) !important;
`;

const ChartWrapper = styled.div``;

const StatsWrapper = styled.div`
  padding: 24px 0 32px;
`;

const DescriptionWrapper = styled.div`
  padding: 32px 0 62px;
`;

const FieldTitle = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;

  color: #000000;
  font-family: TT Firs Neue, sans-serif;
  font-weight: 500;
  font-size: 18px;
  line-height: 140%;
`;

const DescriptionValue = styled.div`
  margin-top: 12px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-size: 16px;
  line-height: 140%;
`;

const DescriptionSiteA = styled.a`
  display: inline-block;
  margin-top: 20px;
  padding-left: 30px;

  color: #5887ff;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 140%;

  background: url('${linkImg}') no-repeat 0 50%;
  background-size: 16px;
`;

const TokenListWrapper = styled.div`
  margin: 24px 0 41px;
`;

const ShowProportions = styled.div`
  color: #5887ff;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 140%;
`;

const TokenList = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fill, 44px);
  grid-gap: 24px;

  margin-top: 24px;
`;

interface Props {}

export const Fund: FC<Props> = (props) => {
  return (
    <Wrapper>
      <Header />
      <Container>
        <Column>
          <BorderBottomWrapper>
            <TopWrapper>
              <FundInfo />
              <LikeButton />
            </TopWrapper>
            <ChartWrapper>Chart</ChartWrapper>
            <StatsWrapper>
              <FundStats />
            </StatsWrapper>
            <DescriptionWrapper>
              <FieldTitle>Fund Description</FieldTitle>
              <DescriptionValue>
                This vault converts your CRV into yveCRV, earning you a
                continuous share of Curve fees. The more converted, the greater
                the rewards. Every week, these can be claimed from the vault as
                3Crv (Curve’s 3pool LP token).
              </DescriptionValue>
              <DescriptionSiteA
                href="https://urlnameexample.com/spfg"
                target="_blank"
                rel="noopener noreferrer noindex"
              >
                https://urlnameexample.com/spfg
              </DescriptionSiteA>
            </DescriptionWrapper>
            <TokenListWrapper>
              <FieldTitle>
                Token list <ShowProportions>Show proportions</ShowProportions>
              </FieldTitle>
              <TokenList>
                {TOKENS['mainnet-beta'].map((token) => (
                  <Avatar src={token.icon} size={44} circle />
                ))}
              </TokenList>
            </TokenListWrapper>
          </BorderBottomWrapper>
        </Column>
        <Column>
          <Invest />
        </Column>
      </Container>
    </Wrapper>
  );
};
