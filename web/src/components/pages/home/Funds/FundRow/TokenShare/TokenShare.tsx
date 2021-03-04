import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { PublicKey } from '@solana/web3.js';
import { shortAddress } from 'utils/common';
import { TOKENS } from 'config/tokens';
import { Avatar } from 'components/common/Avatar';

const Wrapper = styled.div`
  display: flex;
  align-items: center;

  min-width: 108px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-size: 12px;
  line-height: 140%;
`;

const TokenName = styled.div`
  margin-left: 12px;

  font-weight: 600;
`;

const TokenShareValue = styled.div`
  display: inline-block;

  font-weight: 400;
`;

interface Props {
  tokenWeight: { mintAddress: PublicKey; weight: number };
}

export const TokenShare: FC<Props> = ({ tokenWeight }) => {
  const tokenMeta = TOKENS.devnet.find(
    (token) => token.mintAddress === tokenWeight.mintAddress.toBase58(),
  );

  return (
    <Wrapper>
      <Avatar size={18} src={tokenMeta?.icon} />
      <TokenName>
        {tokenMeta?.tokenName ||
          shortAddress(tokenWeight.mintAddress.toBase58())}{' '}
        <TokenShareValue>{tokenWeight.weight}%</TokenShareValue>
      </TokenName>
    </Wrapper>
  );
};
