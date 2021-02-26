import React from 'react';
import { styled } from '@linaria/react';
import { Header } from 'components/common/Header';

const Wrapper = styled.div`
  min-height: 100%;
  padding-bottom: 50px;
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;

  width: 100%;
  max-width: 1068px;
  margin: 0 auto;
`;

export const Home = () => {
  return (
    <Wrapper>
      <Header showConnect />
        <Container>
          Home
        </Container>
    </Wrapper>
  );
};
