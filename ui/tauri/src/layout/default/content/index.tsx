import { Layout } from 'antd';
import { Outlet } from 'react-router-dom';
import classNames from '../index.module.less';

const { Content } = Layout;

export function AppContent() {
  return (
    <Content className={classNames['next-layout-content']}>
      <Outlet />
    </Content>
  );
}
