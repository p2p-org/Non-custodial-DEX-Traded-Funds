import React from 'react';
import { styled } from '@linaria/react';
import { Header } from 'components/common/Header';
import { MainContainer } from 'components/common/MainContainer';
import { TotalInfo } from './TotalInfo';
import { Funds } from './Funds';

const Wrapper = styled.div`
  min-height: 100%;
`;

const Container = styled(MainContainer)`
  display: flex;
  flex-direction: column;

  margin: 80px auto;
`;

const Title = styled.h1`
  color: #000000;
  font-family: TT Firs Neue, sans-serif;
  font-size: 58px;
  font-weight: 500;
  line-height: 100%;
`;

const TotalInfoStyled = styled(TotalInfo)`
  margin-top: 56px;
`;

const FundsStyled = styled(Funds)`
  margin-top: 88px;
`;

export const Home = () => {
  return (
    <Wrapper>
      <Header isMain />
      <Container>
        <Title>First ETF Fund built on Solana network</Title>
        <TotalInfoStyled />
        <FundsStyled />
      </Container>
    </Wrapper>
  );
};
