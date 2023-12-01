import { ConfigProvider } from 'antd';
import { PropsWithChildren } from 'react';

import zhCN from 'antd/locale/zh_CN';

function ApplicationProvider(props: PropsWithChildren) {
  return <ConfigProvider locale={zhCN}>{props.children}</ConfigProvider>;
}

export default ApplicationProvider;
