import classNames from './index.module.less';
import React, { PropsWithChildren } from 'react';

export { TopNavigation } from './src/navigation';

export type PageWrapperProps = PropsWithChildren & {
  contentFullHeight?: boolean;
};

export function PageWrapper(props: PageWrapperProps) {
  const { contentFullHeight, children } = props;

  const pageWrapperClass = React.useMemo(() => {
    let target: string[] = [];

    if (contentFullHeight) {
      target.push(classNames['page-wraper-content-full-height']);
    }

    return target.join(' ');
  }, [contentFullHeight]);

  return <div className={pageWrapperClass}>{children}</div>;
}
