import { Layout } from 'antd';
import classNames from './index.module.less';
import { AppSider } from './sider';
import { AppContent } from './content';

function AppLayout() {
  return (
    <Layout className={classNames['next-layout-default']}>
      <Layout>
        <AppSider />
        <Layout className={classNames['next-layout-main']}>
          <AppContent />
        </Layout>
      </Layout>
    </Layout>
  );
}

export default AppLayout;
