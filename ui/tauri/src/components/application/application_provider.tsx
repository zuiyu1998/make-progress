import { ConfigProvider } from 'antd';
import { PropsWithChildren } from 'react';

import zhCN from 'antd/locale/zh_CN';
import 'dayjs/locale/zh-cn';

function ApplicationProvider(props: PropsWithChildren) {
  return <ConfigProvider locale={zhCN}>{props.children}</ConfigProvider>;
}

export default ApplicationProvider;
