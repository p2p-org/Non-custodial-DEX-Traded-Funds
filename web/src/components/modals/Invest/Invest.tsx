import React, { FC, useMemo } from 'react';

import { styled } from '@linaria/react';
import { useGate, useStore } from 'effector-react';
import BN from 'bn.js';
import { Decimal } from 'decimal.js';
import { Modal } from 'components/common/Modal';
import { Button } from 'components/ui/Button';
import { Avatar } from 'components/common/Avatar';
import { Input } from 'components/ui/Input';
import { shortAddress } from 'utils/common';
import { FundType } from '../../../models/types';
import { FUNDS } from '../../../config/funds';
import {
  $amount,
  $baseTokenAccount,
  $isLoading,
  changeAmount,
  investClicked,
  InvestGate,
  setAmount,
} from './model';

const WrapperModal = styled(Modal)`
  flex-basis: 512px;
`;

const Wrapper = styled.div`
  & > :not(:last-child) {
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  }
`;

const InvestWrapper = styled.div`
  padding: 0 20px;
`;

const FundInfoWrapper = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;

  padding: 32px 0;
`;

const FundName = styled.div`
  margin-top: 8px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 24px;
`;

const InputsWrapper = styled.div`
  & > :not(:last-child) {
    margin-bottom: 24px;
  }
`;

const BalanceWrapper = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;

  padding: 12px 0 38px;
`;

const OptionsWrapper = styled.div`
  display: flex;

  & > :not(:last-child) {
    margin-right: 10px;
  }
`;

const AvailableBalance = styled.div`
  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 24px;

  cursor: pointer;
`;

const Option = styled.div`
  display: flex;
  align-items: center;
  height: 34px;
  padding: 0 15px;

  color: #a3a5ba;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 14px;
  line-height: 100%;

  background: #fbfbfd;
  border-radius: 12px;
  cursor: pointer;
`;

const TransactionInfoWrapper = styled.div`
  padding: 20px;
`;

const InfoLine = styled.div`
  display: flex;
  justify-content: space-between;

  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 24px;

  &:not(:last-child) {
    margin-bottom: 8px;
  }
`;

const InfoLeft = styled.div`
  color: #a3a5ba;
`;

const InfoRight = styled.div`
  color: #000000;
`;

const Footer = styled.div`
  padding: 24px 20px;
`;

const InvestButton = styled(Button)`
  height: 56px;

  font-weight: 600;
  font-size: 16px;
  line-height: 140%;
`;

interface Props {
  fund: FundType;
  close: () => void;
}

export const Invest: FC<Props> = ({ fund, close }) => {
  useGate(InvestGate, fund);
  const baseTokenAccount = useStore($baseTokenAccount);
  const amount = useStore($amount);
  const isLoading = useStore($isLoading);

  const fundMeta = useMemo(
    () =>
      FUNDS.devnet.find(
        (fundItem) => fundItem.address === fund.pubkey.toBase58(),
      ),
    [fund],
  );

  const handleAllBalanceClick = () => {
    if (!baseTokenAccount) {
      return;
    }

    setAmount(new Decimal(baseTokenAccount.account.data.tokenAmount.uiAmount));
  };

  return (
    <WrapperModal title="Invest" close={close}>
      <Wrapper>
        <InvestWrapper>
          <FundInfoWrapper>
            <Avatar size={64} src={fundMeta?.icon} />
            <FundName>
              {fundMeta
                ? `${fundMeta.fundName} (${fundMeta.fundSymbol})`
                : fund.account.data.name}
            </FundName>
          </FundInfoWrapper>
          <InputsWrapper>
            <Input
              prefix="Buy"
              postfix={
                fundMeta
                  ? fundMeta.fundSymbol
                  : shortAddress(fund.account.data.poolTokenMint.toBase58())
              }
              placeholder="0.0"
            />
            <Input
              prefix="Pay"
              postfix="USDC"
              placeholder="0.0"
              value={amount.toString()}
              onChange={changeAmount}
            />
          </InputsWrapper>
          {baseTokenAccount ? (
            <BalanceWrapper>
              <OptionsWrapper>
                <Option onClick={handleAllBalanceClick}>
                  Max: {baseTokenAccount.account.data.tokenAmount.uiAmount}
                </Option>
                <Option
                  onClick={() =>
                    setAmount(
                      new Decimal(
                        baseTokenAccount.account.data.tokenAmount.uiAmount / 2,
                      ),
                    )
                  }
                >
                  1/2
                </Option>
                <Option
                  onClick={() =>
                    setAmount(
                      new Decimal(
                        baseTokenAccount.account.data.tokenAmount.uiAmount / 4,
                      ),
                    )
                  }
                >
                  1/4
                </Option>
              </OptionsWrapper>
              <AvailableBalance onClick={handleAllBalanceClick}>
                Balance: {baseTokenAccount.account.data.tokenAmount.uiAmount}{' '}
                USDC
              </AvailableBalance>
            </BalanceWrapper>
          ) : null}
        </InvestWrapper>
        <TransactionInfoWrapper>
          <InfoLine>
            <InfoLeft>You’ll receive:</InfoLeft>
            <InfoRight>
              21.1507{' '}
              {fundMeta
                ? fundMeta.fundSymbol
                : shortAddress(fund.account.data.poolTokenMint.toBase58())}
            </InfoRight>
          </InfoLine>
          <InfoLine>
            <InfoLeft>Transaction Fee:</InfoLeft>
            <InfoRight>0.00012 SOL</InfoRight>
          </InfoLine>
        </TransactionInfoWrapper>
        <Footer>
          <InvestButton
            full
            primary
            onClick={investClicked}
            disabled={isLoading}
          >
            {isLoading ? 'Loading...' : 'Invest now'}gst
          </InvestButton>
        </Footer>
      </Wrapper>
    </WrapperModal>
  );
};
