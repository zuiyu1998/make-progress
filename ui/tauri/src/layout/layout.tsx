import { Layout } from 'antd';
import { PropsWithChildren } from 'react';
import classNames from './index.module.less';
import { AppSider } from './sider';

const { Content } = Layout;

function AppLayout(props: PropsWithChildren) {
  const { children } = props;

  return (
    <Layout className={classNames.layout}>
      <Layout>
        <AppSider />
        <Layout>
          <Content>{children}</Content>
        </Layout>
      </Layout>
    </Layout>
  );
}

export default AppLayout;
