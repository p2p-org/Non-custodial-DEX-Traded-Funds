import React, { FC } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';
import { Button } from 'components/ui/Button';

import { Avatar } from 'components/common/Avatar';
import { openModalFx } from 'models/modals';
import {
  MODAL_INVEST,
  MODAL_WITHDRAW,
} from 'components/common/ModalManager/constants';
import { Column } from '../common/Column';

const TopWrapper = styled.div`
  display: flex;
  flex-flow: nowrap;

  padding: 24px;
`;

const InfoWrapper = styled.div`
  margin-left: 12px;
`;

const FundName = styled.h4`
  margin-bottom: 4px;

  color: #5887ff;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 140%;
`;

const FundDate = styled.div`
  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-size: 12px;
  line-height: 140%;
`;

const ColumnValue = styled(Column)`
  flex-direction: column;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 14px;
  line-height: 140%;

  &.price {
    font-weight: 700;
  }

  &.alert {
    color: #ffa631;
  }

  &.profit {
    color: #62cf7a;
  }
`;

const AdditionalInfo = styled.div`
  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-size: 13px;
  line-height: 140%;
`;

const ColumnButtons = styled.div`
  opacity: 0;

  & > :not(:last-child) {
    margin-right: 16px;
  }
`;

const Wrapper = styled.div`
  background: #ffffff;
  box-shadow: 0 8px 8px rgba(0, 0, 0, 0.03);
  border-radius: 12px;

  cursor: pointer;

  transition: box-shadow 100ms cubic-bezier(0.64, 0, 0.35, 1) 0s;

  &:hover {
    box-shadow: 0 8px 24px #d1d1d1;

    ${ColumnButtons} {
      opacity: 1;

      visibility: visible;
    }
  }
`;

const ButtonStyled = styled(Button)`
  padding: 0 16px;
`;

const BottomWrapper = styled.div`
  display: grid;
  grid-auto-columns: 1fr max-content;
  grid-auto-flow: column;

  padding: 16px 24px;

  border-top: 1px solid rgba(163, 165, 186, 0.2);
`;

const TokenSharesRow = styled.div`
  display: flex;
  justify-content: space-between;
`;

const TokenShare = styled.div`
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

interface Props {}

export const FundRow: FC<Props> = (props) => {
  const handleOpenInvestModalClick = () => {
    openModalFx({ modalType: MODAL_INVEST });
  };

  const handleOpenWithdrawModalClick = (
    e: React.MouseEvent<HTMLButtonElement>,
  ) => {
    e.stopPropagation();

    openModalFx({ modalType: MODAL_WITHDRAW });
  };

  return (
    <Wrapper onClick={handleOpenInvestModalClick}>
      <TopWrapper>
        <Column className={classNames({ name: true })}>
          <Avatar />
          <InfoWrapper>
            <FundName>Alameda Bull DTF (ABDFS)</FundName>
            <FundDate>Inception date: Feb 26, 2021</FundDate>
          </InfoWrapper>
        </Column>
        <ColumnValue className={classNames({ marketCap: true })}>
          $12,000,000.21
        </ColumnValue>
        <ColumnValue className={classNames({ price: true })}>
          $124.91
        </ColumnValue>
        <ColumnValue className={classNames({ since: true, profit: true })}>
          + 420.01%
        </ColumnValue>
        <ColumnValue className={classNames({ balance: true })}>
          <div>48.00 ABDFS</div>
          <AdditionalInfo>Valuation: $5952</AdditionalInfo>
        </ColumnValue>

        <ColumnButtons>
          <ButtonStyled
            primary
            disabled={false}
            onClick={handleOpenWithdrawModalClick}
          >
            Withdraw
          </ButtonStyled>
          <ButtonStyled primary>+ Invest</ButtonStyled>
        </ColumnButtons>
      </TopWrapper>
      <BottomWrapper>
        <TokenSharesRow>
          <TokenShare>
            <Avatar size={18} />
            <TokenName>
              FTT <TokenShareValue>33.11%</TokenShareValue>
            </TokenName>
          </TokenShare>
          <TokenShare>
            <Avatar size={18} />
            <TokenName>
              FTT <TokenShareValue>33.11%</TokenShareValue>
            </TokenName>
          </TokenShare>
          <TokenShare>
            <Avatar size={18} />
            <TokenName>
              FTT <TokenShareValue>33.11%</TokenShareValue>
            </TokenName>
          </TokenShare>
          <TokenShare>
            <Avatar size={18} />
            <TokenName>
              FTT <TokenShareValue>33.11%</TokenShareValue>
            </TokenName>
          </TokenShare>
          <TokenShare>
            <Avatar size={18} />
            <TokenName>
              FTT <TokenShareValue>33.11%</TokenShareValue>
            </TokenName>
          </TokenShare>
          <TokenShare>
            <Avatar size={18} />
            <TokenName>
              FTT <TokenShareValue>33.11%</TokenShareValue>
            </TokenName>
          </TokenShare>
        </TokenSharesRow>
      </BottomWrapper>
    </Wrapper>
  );
};
