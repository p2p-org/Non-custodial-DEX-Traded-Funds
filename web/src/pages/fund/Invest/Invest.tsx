import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { Input } from 'components/ui/Input';
import { Button } from 'components/ui/Button';

const Wrapper = styled.div`
  background: #ffffff;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.05);
  border-radius: 4px;
`;

const Section = styled.div`
  padding: 24px 24px 32px;

  &:not(:last-child) {
    border-bottom: 1px solid rgba(163, 165, 186, 0.2);
  }
`;

const Title = styled.div`
  margin-bottom: 15px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 140%;
`;

const TitleValue = styled.div`
  display: inline-block;

  color: #5887ff;
`;

const Group = styled.div`
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: 1fr 174px;
  grid-gap: 16px;
`;

const InputStyled = styled(Input)`
  background: #fbfbfd;

  input {
    font-family: TT Firs Neue, sans-serif;
    font-weight: 500;
  }
`;

const Description = styled.div`
  margin-top: 28px;

  color: #8c8e99;
  font-family: Titillium Web, sans-serif;
  font-size: 14px;
  line-height: 140%;
`;

interface Props {}

export const Invest: FC<Props> = (props) => {
  return (
    <Wrapper>
      <Section>
        <Title>
          Your wallet <TitleValue>1348.00 USDC</TitleValue>
        </Title>
        <Group>
          <InputStyled />
          <Button primary>+ Invest now</Button>
        </Group>
      </Section>
      <Section>
        <Title>
          Your balance <TitleValue>48.00 Tokens</TitleValue>
        </Title>
        <Group>
          <InputStyled />
          <Button primary disabled>
            Withdraw
          </Button>
        </Group>
        <Description>
          This vault converts your CRV into yveCRV, earning you a continuous
          share of Curve fees. The more converted, the greater the rewards.
          Every week, these can be claimed from the vault as 3Crv (Curve’s 3pool
          LP token).
        </Description>
      </Section>
    </Wrapper>
  );
};
