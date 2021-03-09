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
import { moneyFormat } from 'utils/common';
import { TOKENS } from 'config/tokens';
import { FUNDS } from 'config/funds';
import { $ratesMap } from 'models/rates';
import { PoolStatePopulated } from 'models/types';
import { $connected } from 'models/wallet';
import { Column } from '../common/Column';
import { TokenShare } from './TokenShare';

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

  &.isHoverable {
    cursor: pointer;

    transition: box-shadow 100ms cubic-bezier(0.64, 0, 0.35, 1) 0s;

    &:hover {
      box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);

      ${ColumnButtons} {
        opacity: 1;

        visibility: visible;
      }
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

interface Props {
  fund: PublicKeyAndAccount<PoolStatePopulated>;
}

// eslint-disable-next-line sonarjs/cognitive-complexity
export const FundRow: FC<Props> = ({ fund }) => {
  const connected = useStore($connected);
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

    return assetWeights
      .map((weight, index) => ({
        mintAddress: assets[index].mint,
        weight: weight / 10,
      }))
      .sort((a, b) => b.weight - a.weight);
  }, [fund]);

  const handleOpenInvestModalClick = () => {
    openModalFx({
      modalType: MODAL_INVEST,
      props: { fund },
    });
  };

  const handleOpenWithdrawModalClick = (
    e: React.MouseEvent<HTMLButtonElement>,
  ) => {
    e.stopPropagation();

    openModalFx({
      modalType: MODAL_WITHDRAW,
      props: { fund },
    });
  };

  return (
    <Wrapper
      onClick={connected ? handleOpenInvestModalClick : undefined}
      className={classNames({ isHoverable: connected })}
    >
      <TopWrapper>
        <Column className={classNames({ name: true })}>
          <Avatar src={fundMeta?.icon} />
          <InfoWrapper>
            {/* TODO: original name*/}
            <FundName>
              {fundMeta
                ? `${fundMeta.fundName} (${fundMeta.fundSymbol})`
                : fund.account.data.name}
            </FundName>
            <FundDate>Inception date: Feb 26, 2021</FundDate>
          </InfoWrapper>
        </Column>
        <ColumnValue
          title={String(fundMarketCap || '')}
          className={classNames({ marketCap: true })}
        >
          {moneyFormat(fundMarketCap)}
        </ColumnValue>
        <ColumnValue
          title={String(fundPrice || '')}
          className={classNames({ price: true })}
        >
          {moneyFormat(fundPrice)}
        </ColumnValue>
        <ColumnValue className={classNames({ since: true, profit: true })}>
          + 420.01%
        </ColumnValue>
        {connected ? (
          <>
            <ColumnValue className={classNames({ balance: true })}>
              <div>48.00 {fundMeta?.fundSymbol}</div>
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
          </>
        ) : null}
      </TopWrapper>
      <BottomWrapper>
        <TokenSharesRow>
          {tokens.map((tokenWeight) => (
            <TokenShare
              key={tokenWeight.mintAddress.toBase58()}
              tokenWeight={tokenWeight}
            />
          ))}
        </TokenSharesRow>
      </BottomWrapper>
    </Wrapper>
  );
};
