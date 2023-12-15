import classNames from './index.module.less';

export type BakgroundProps = {
  background?: string;
  alt: string;
};

export function Bakground(props: BakgroundProps) {
  const { background, alt } = props;

  return (
    <div className={classNames['background-container']}>
      {background ? (
        <div>
          <img src={background} />
        </div>
      ) : (
        <div>
          <div>{alt.slice(0, 1)}</div>
        </div>
      )}
    </div>
  );
}
