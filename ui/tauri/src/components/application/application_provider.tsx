import { ConfigProvider } from 'antd';
import { PropsWithChildren } from 'react';
import { App } from 'antd';
import zhCN from 'antd/locale/zh_CN';
import 'dayjs/locale/zh-cn';
import classNames from './index.module.less';

function ApplicationProvider(props: PropsWithChildren) {
  return (
    <ConfigProvider locale={zhCN}>
      <App className={classNames['antd-application']}>{props.children}</App>
    </ConfigProvider>
  );
}

export default ApplicationProvider;
