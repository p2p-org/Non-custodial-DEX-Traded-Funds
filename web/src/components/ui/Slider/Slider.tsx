import React, { PureComponent, createRef } from 'react';
import { styled } from '@linaria/react';
import classNames from 'classnames';
import hamburgerImg from './hamburger.png';

const Progress = styled.div`
  position: absolute;

  top: 15px;
  left: 0;

  height: 4px;

  border-radius: 2px;
  background: #5887ff;
`;

const HandleSlot = styled.div`
  position: relative;

  margin: 0 11px;
`;

const HandleWrapper = styled.div`
  position: absolute;

  padding: 5px;
  margin: -5px 0 0 -16px;
`;

const Handle = styled.div`
  width: 32px;
  height: 32px;

  color: #fff;

  background: #ffffff url('${hamburgerImg}') no-repeat 50% 50%;
  background-size: 12px 7px;
  border: 1px solid rgba(163, 165, 186, 0.5);
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);

  cursor: pointer;
`;

const HandleValue = styled.div`
  width: 45px;
  margin-left: -5px;
  margin-top: 8px;

  color: #000000;
  font-family: Titillium Web, sans-serif;
  font-weight: 600;
  font-size: 16px;
  line-height: 24px;
  text-align: center;
`;

const Wrapper = styled.div`
  position: relative;

  height: 64px;
  user-select: none;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;

  .disabled {
    cursor: default;

    ${Progress} {
      background: #8e8e8e;
    }

    ${Handle} {
      border-color: #8e8e8e;
      background: #8e8e8e;
      cursor: default;
    }
  }

  &:before {
    position: absolute;
    content: '';

    top: 15px;
    right: 0;
    left: 0;

    height: 4px;

    border-radius: 2px;
    background: #a3a5ba;
  }
`;

interface Props {
  value: number | string;
  min: number;
  max: number;
  disabled?: boolean;
  onChange: (nextValue: number) => void;
}

export class Slider extends PureComponent<Props> {
  static defaultProps = {
    value: 0,
    min: 0,
    max: 100,
  };

  rootRef = createRef<HTMLDivElement>();

  isListenerActive = false;

  componentWillUnmount() {
    this.removeListeners();
  }

  removeListeners() {
    if (this.isListenerActive) {
      this.isListenerActive = false;
      window.removeEventListener('mousemove', this.onMove);
      window.removeEventListener('mouseup', this.onMovingEnd);
      window.removeEventListener('touchmove', this.onMove);
      window.removeEventListener('touchend', this.onMovingEnd);
      window.removeEventListener('visibilitychange', this.onVisibilityChange);
    }
  }

  calculateValue(
    e:
      | React.MouseEvent<HTMLDivElement>
      | React.TouchEvent<HTMLDivElement>
      | MouseEvent
      | TouchEvent,
  ) {
    if (!this.rootRef.current) {
      return 0;
    }

    let { clientX } = e as React.MouseEvent<HTMLDivElement>;
    if (!clientX && (e as TouchEvent).changedTouches) {
      clientX = (e as TouchEvent).changedTouches[0].clientX;
    }

    const { min, max } = this.props;
    const box = this.rootRef.current.getBoundingClientRect();
    const unbound = Math.round(
      min + ((max - min) * (clientX - box.left)) / box.width,
    );

    return Math.min(max, Math.max(min, unbound));
  }

  resetMoving() {
    this.removeListeners();
  }

  onClick = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();

    this.setState({
      value: this.calculateValue(e),
    });
  };

  onMouseDown = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();

    this.setState({
      value: this.calculateValue(e),
    });

    if (!this.isListenerActive) {
      this.isListenerActive = true;
      window.addEventListener('mousemove', this.onMove);
      window.addEventListener('mouseup', this.onMovingEnd);
      window.addEventListener('visibilitychange', this.onVisibilityChange);
    }
  };

  onTouchStart = (e: React.TouchEvent<HTMLDivElement>) => {
    e.preventDefault();

    this.setState({
      value: this.calculateValue(e),
    });

    if (!this.isListenerActive) {
      this.isListenerActive = true;
      window.addEventListener('touchmove', this.onMove);
      window.addEventListener('touchend', this.onMovingEnd);
      window.addEventListener('visibilitychange', this.onVisibilityChange);
    }
  };

  onMove = (e: MouseEvent | TouchEvent) => {
    e.preventDefault();

    this.props.onChange(this.calculateValue(e));
  };

  onMovingEnd = (e: MouseEvent | TouchEvent) => {
    e.preventDefault();

    this.resetMoving();
    this.props.onChange(this.calculateValue(e));
  };

  onVisibilityChange = () => {
    if (document.hidden) {
      this.resetMoving();
    }
  };

  render() {
    const { min, max, disabled } = this.props;
    const value = Number(this.props.value);
    const percent = (100 * (value - min)) / (max - min) || 0;

    return (
      <Wrapper
        onClick={disabled ? undefined : this.onClick}
        onMouseDown={disabled ? undefined : this.onMouseDown}
        onTouchStart={disabled ? undefined : this.onTouchStart}
        className={classNames({ disabled })}
      >
        <Progress style={{ width: `${percent}%` }} />
        <HandleSlot ref={this.rootRef}>
          <HandleWrapper style={{ left: `${percent}%` }}>
            <Handle />
            <HandleValue>{percent}%</HandleValue>
          </HandleWrapper>
        </HandleSlot>
      </Wrapper>
    );
  }
}
