import React, { CSSProperties } from 'react';
import classNames from './index.module.less';

export type BakgroundProps = {
  background?: string;
  alt: string;
};

const colors = ['red'];

function useBackgroundColor(len: number) {
  return React.useMemo(() => {
    let index = len % colors.length;
    return colors[index];
  }, [len]);
}

export function Bakground(props: BakgroundProps) {
  const { background, alt } = props;

  const backgroundColor = useBackgroundColor(alt.length);

  const contentCss: CSSProperties = React.useMemo(() => {
    return {
      background: backgroundColor,
    };
  }, [backgroundColor]);

  return (
    <div className={classNames['background-container']}>
      {background ? (
        <div>
          <img src={background} />
        </div>
      ) : (
        <div
          className={classNames['background-container-alt-content']}
          style={contentCss}
        >
          <div className={classNames['background-text']}>{alt.slice(0, 1)}</div>
        </div>
      )}
    </div>
  );
}
