import React, { FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';
import { Link } from 'react-router-dom';
import { Button } from 'components/ui/Button';

import { FundInfo } from 'components/common/FundInfo';
import { LikeButton } from 'components/common/LikeButton';
import likeImg from '../../../../components/common/LikeButton/like.png';
import { Fund } from '../../../fund';
import { FundStats } from '../../../../components/common/FundStats';

const Wrapper = styled.div`
  background: #ffffff;
  border-radius: 4px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.05);
`;

const TopWrapper = styled.div`
  display: flex;
  justify-content: space-between;

  padding: 24px;
`;

const BottomWrapper = styled.div`
  display: grid;
  grid-auto-columns: 1fr max-content;
  grid-auto-flow: column;

  padding: 16px 24px;

  border-top: 1px solid rgba(163, 165, 186, 0.2);
`;

interface Props {}

export const FundRow: FC<Props> = (props) => {
  return (
    <Link to="/somefundaddress">
      <Wrapper>
        <TopWrapper>
          <FundInfo />
          <Link to="/somefundaddress">
            <Button primary>+ Invest now</Button>
          </Link>
        </TopWrapper>
        <BottomWrapper>
          <FundStats isFundRow />
          <LikeButton />
        </BottomWrapper>
      </Wrapper>
    </Link>
  );
};
