import { Layout } from 'antd';
import { PropsWithChildren } from 'react';
import classNames from './index.module.less';

const { Sider, Content } = Layout;

function AppLayout(props: PropsWithChildren) {
  const { children } = props;

  return (
    <Layout className={classNames.layout}>
      <Layout>
        <Sider trigger={null} collapsible collapsed={true}></Sider>
        <Layout>
          <Content>{children}</Content>
        </Layout>
      </Layout>
    </Layout>
  );
}

export default AppLayout;
