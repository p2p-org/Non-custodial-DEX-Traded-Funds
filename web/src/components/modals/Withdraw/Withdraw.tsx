import React, { FC, useState } from 'react';

import { styled } from '@linaria/react';
import { Modal } from 'components/common/Modal';
import { Button } from 'components/ui/Button';
import { Avatar } from 'components/common/Avatar';
import { Input } from 'components/ui/Input';
import { Slider } from 'components/ui/Slider';

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
  margin-bottom: 32px;
`;

const AvailableBalanceWrapper = styled.div`
  display: flex;
  justify-content: flex-end;

  margin: 16px 0 25px;
`;

const AvailableBalance = styled.div`
  color: #5887ff;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 24px;

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
  close: () => void;
}

export const Withdraw: FC<Props> = ({ close }) => {
  const [percentage, setPercentage] = useState(0);

  const handlePercentageChange = (nextValue: number) => {
    setPercentage(nextValue);
  };

  return (
    <WrapperModal title="Withdraw" close={close}>
      <Wrapper>
        <InvestWrapper>
          <FundInfoWrapper>
            <Avatar size={64} />
            <FundName>ABDFS</FundName>
          </FundInfoWrapper>
          <InputsWrapper>
            <Input prefix="Amount" postfix="ABDFS" placeholder="0.0" />
            <AvailableBalanceWrapper>
              <AvailableBalance>
                Available balance: 21.1507 ABDFS
              </AvailableBalance>
            </AvailableBalanceWrapper>
            <Slider
              value={percentage}
              min={0}
              max={100}
              onChange={handlePercentageChange}
            />
          </InputsWrapper>
        </InvestWrapper>
        <TransactionInfoWrapper>
          <InfoLine>
            <InfoLeft>You’ll receive:</InfoLeft>
            <InfoRight>21.1507 ABDFS</InfoRight>
          </InfoLine>
          <InfoLine>
            <InfoLeft>Transaction Fee:</InfoLeft>
            <InfoRight>0.00012 SOL</InfoRight>
          </InfoLine>
        </TransactionInfoWrapper>
        <Footer>
          <InvestButton full primary>
            Withdraw now
          </InvestButton>
        </Footer>
      </Wrapper>
    </WrapperModal>
  );
};
