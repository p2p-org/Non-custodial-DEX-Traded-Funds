import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { useStore } from 'effector-react';
import { $connected, $wallet } from 'models/wallet';
import { Button } from 'components/ui/Button';
import { connectClicked } from './model';

const Wrapper = styled.div`
  display: flex;
  align-items: center;

  height: 88px;

  background: #fff;
`;

const Container = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;

  width: 100%;
  max-width: 1340px;
  margin: 0 auto;
`;

const Right = styled.div`
  display: flex;
`;

export const Header: FC = () => {
  const connected = useStore($connected);
  const wallet = useStore($wallet);

  return (
    <Wrapper>
      <Container>
        <div>Logo</div>
        <Right>
          {!connected ? (
            <Button onClick={connectClicked}>Connect</Button>
          ) : wallet}
        </Right>
      </Container>
    </Wrapper>
  );
};
