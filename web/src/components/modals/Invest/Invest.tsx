import React, { FC } from 'react';

import { styled } from '@linaria/react';
import { Modal } from 'components/common/Modal';
import { Button } from 'components/ui/Button';
import { Avatar } from 'components/common/Avatar';
import { Input } from 'components/ui/Input';

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

const OptionsWrapper = styled.div`
  display: flex;
  justify-content: center;

  padding: 12px 0 38px;

  & > :not(:last-child) {
    margin-right: 10px;
  }
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
  close: () => void;
}

export const Invest: FC<Props> = ({ close }) => {
  return (
    <WrapperModal title="Invest" close={close}>
      <Wrapper>
        <InvestWrapper>
          <FundInfoWrapper>
            <Avatar size={64} />
            <FundName>ABDFS</FundName>
          </FundInfoWrapper>
          <InputsWrapper>
            <Input prefix="Buy" postfix="ABDFS" placeholder="0.0" />
            <Input prefix="Pay" postfix="USDC" placeholder="0.0" />
          </InputsWrapper>
          <OptionsWrapper>
            <Option>Max: 2665.708</Option>
            <Option>1/2</Option>
            <Option>$1/4</Option>
          </OptionsWrapper>
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
            Invest now
          </InvestButton>
        </Footer>
      </Wrapper>
    </WrapperModal>
  );
};
