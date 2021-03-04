import React, { FunctionComponent, useEffect, useRef, useState } from 'react';

import { styled } from '@linaria/react';
import { Cluster } from '@solana/web3.js';
import classNames from 'classnames';

import { Icon } from 'components/ui/Icon';

import { ItemRow } from './ItemRow';

const Wrapper = styled.div`
  position: relative;

  background: #f6f6f8;
  border-radius: 12px;
`;

const SelectorWrapper = styled.div`
  display: flex;
  align-items: center;
  max-width: 153px;
  margin: 4px;

  cursor: pointer;

  &.isOpen,
  &:hover {
  }
`;

const ArrowWrapper = styled.div`
  margin-right: 4px;
`;

const ArrowIcon = styled(Icon)`
  width: 16px;
  height: 16px;

  color: #a3a5ba;
`;

const DropDownList = styled.div`
  position: absolute;
  right: 0;
  z-index: 1;

  min-width: 204px;
  margin-top: 8px;
  padding: 8px;

  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
`;

export const Selector: FunctionComponent = ({ children }) => {
  const selectorRef = useRef<HTMLDivElement>(null);
  const [isOpen, setIsOpen] = useState(false);

  const handleAwayClick = (e: MouseEvent) => {
    if (!selectorRef.current?.contains(e.target as HTMLDivElement)) {
      setIsOpen(false);
    }
  };

  useEffect(() => {
    window.addEventListener('click', handleAwayClick);

    return () => {
      window.removeEventListener('click', handleAwayClick);
    };
  }, []);

  const handleSelectorClick = () => {
    setIsOpen(!isOpen);
  };

  const handleItemClick = (newCluster: Cluster) => {
    setIsOpen(false);
  };

  return (
    <Wrapper ref={selectorRef}>
      <SelectorWrapper
        onClick={handleSelectorClick}
        className={classNames({ isOpen })}
      >
        {children}
        <ArrowWrapper>
          <ArrowIcon name="arrow-triangle" />
        </ArrowWrapper>
      </SelectorWrapper>
      {isOpen ? (
        <DropDownList>
          {/*<ItemRow*/}
          {/*  key={itemCluster}*/}
          {/*  isSelected={cluster === itemCluster}*/}
          {/*  cluster={itemCluster}*/}
          {/*  onItemClick={handleItemClick}*/}
          {/*>*/}
          {/*  {itemCluster}*/}
          {/*</ItemRow>*/}
        </DropDownList>
      ) : undefined}
    </Wrapper>
  );
};
