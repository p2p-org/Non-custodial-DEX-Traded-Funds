import React from 'react';
import { styled } from '@linaria/react';
import { Header } from 'components/common/Header';
import { MainContainer } from 'components/common/MainContainer';
import { Funds } from '../../components/pages/home/Funds';

const Wrapper = styled.div`
  min-height: 100%;
`;

const Container = styled(MainContainer)`
  display: flex;
  flex-direction: column;

  margin: 100px auto;
`;

const Title = styled.h1`
  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 62px;
  line-height: 120%;
`;

const Description = styled.div`
  margin-top: 12px;

  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 20px;
  line-height: 120%;

  text-align: center;
`;

const FundsStyled = styled(Funds)`
  margin-top: 85px;
`;

export const Home = () => {
  return (
    <Wrapper>
      <Header />
      <Container>
        <Title>Non-custodial DEX Traded Funds(DTFs)</Title>
        <Description>Build on top of Solana and Serum</Description>
        <FundsStyled />
      </Container>
    </Wrapper>
  );
};
