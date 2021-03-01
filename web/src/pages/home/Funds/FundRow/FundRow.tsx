import React, { FC, useMemo } from 'react';

import { styled } from '@linaria/react';
import classNames from 'classnames';
import { PublicKeyAndAccount } from '@solana/web3.js';
import { useStore } from 'effector-react';
import { Button } from 'components/ui/Button';

import { Avatar } from 'components/common/Avatar';
import { openModalFx } from 'models/modals';
import {
  MODAL_INVEST,
  MODAL_WITHDRAW,
} from 'components/common/ModalManager/constants';
import { shortAddress } from 'utils/common';
import { TOKENS } from 'config/tokens';
import { FUNDS } from 'config/funds';
import { PoolState } from '../../../../../../js/lib/fund';
import { Column } from '../common/Column';
import { $ratesMap } from '../../../../models/rates';
import { PoolStatePopulated } from '../../../../models/connection/types';

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
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);

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

interface Props {
  fund: PublicKeyAndAccount<PoolStatePopulated>;
}

export const FundRow: FC<Props> = ({ fund }) => {
  const ratesMap = useStore($ratesMap);

  const fundMarketCap = useMemo(
    () =>
      fund.account.data.assets.reduce((marketCap, asset) => {
        if (!asset.vaultPopulated) {
          console.error(
            'vaultPopulated for marketCap calculation did not found:',
            asset.mint.toBase58(),
          );
          return marketCap;
        }

        const tokenMeta = TOKENS.devnet.find(
          (token) => token.mintAddress === asset.mint.toBase58(),
        );
        if (!tokenMeta) {
          console.error(
            'tokenMeta for marketCap calculation did not found:',
            asset.mint.toBase58(),
          );
          return marketCap;
        }

        const rate = ratesMap[tokenMeta.tokenSymbol];

        if (!rate) {
          console.error(
            'rate for marketCap calculation did not found:',
            tokenMeta.tokenSymbol,
          );
          return marketCap;
        }

        return marketCap + asset.vaultPopulated.tokenAmount.uiAmount * rate;
      }, 0),
    [fund, ratesMap],
  );

  const fundPrice = useMemo(() => {
    if (!fund.account.data.poolTokenMintPopulated?.supply) {
      console.error(
        'supply for fundPrice calculation did not found:',
        fund.account.data.poolTokenMint.toBase58(),
      );
      return 0;
    }

    // TODO: BN
    return (
      fundMarketCap / Number(fund.account.data.poolTokenMintPopulated.supply)
    );
  }, [fund, fundMarketCap]);

  const fundMeta = useMemo(
    () =>
      FUNDS.devnet.find(
        (fundItem) => fundItem.address === fund.pubkey.toBase58(),
      ),
    [fund],
  );

  const tokens = useMemo(() => {
    const { assets, fundState } = fund.account.data;

    const assetWeights = fundState?.assetWeights;

    if (!assetWeights) {
      return [];
    }

    return assetWeights.map((weight, index) => ({
      mintAddress: assets[index].mint,
      weight: weight / 10,
    }));
  }, [fund]);

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
          <Avatar src={fundMeta?.icon} />
          <InfoWrapper>
            <FundName>{fundMeta?.fundName}</FundName>
            <FundDate>Inception date: Feb 26, 2021</FundDate>
          </InfoWrapper>
        </Column>
        <ColumnValue
          title={String(fundMarketCap || '')}
          className={classNames({ marketCap: true })}
        >
          {new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD',
          }).format(fundMarketCap)}
        </ColumnValue>
        <ColumnValue
          title={String(fundPrice || '')}
          className={classNames({ price: true })}
        >
          {new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD',
          }).format(fundPrice)}
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
          {tokens.map((tokenWeight) => {
            const tokenMeta = TOKENS.devnet.find(
              (token) =>
                token.mintAddress === tokenWeight.mintAddress.toBase58(),
            );

            return (
              <TokenShare key={tokenWeight.mintAddress.toBase58()}>
                <Avatar size={18} src={tokenMeta?.icon} />
                <TokenName>
                  {tokenMeta?.tokenName ||
                    shortAddress(tokenWeight.mintAddress.toBase58())}{' '}
                  <TokenShareValue>{tokenWeight.weight}%</TokenShareValue>
                </TokenName>
              </TokenShare>
            );
          })}
        </TokenSharesRow>
      </BottomWrapper>
    </Wrapper>
  );
};
