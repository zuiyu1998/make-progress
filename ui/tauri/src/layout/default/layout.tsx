import { Layout } from 'antd';
import classNames from './index.module.less';
import { AppSider } from './sider';
import { Outlet } from 'react-router-dom';

const { Content } = Layout;

function AppLayout() {
  return (
    <Layout className={classNames.layout}>
      <Layout>
        <AppSider />
        <Layout>
          <Content>
            <Outlet />
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
}

export default AppLayout;
