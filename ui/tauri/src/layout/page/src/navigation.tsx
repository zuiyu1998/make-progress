import { Layout, Button } from 'antd';
import { PropsWithChildren } from 'react';
import classNames from '../index.module.less';
import Icon, { IconClass } from '/@/components/icon';

const { Header, Content } = Layout;

export type TopNavigationProps = {
  onBack?: () => void;
} & PropsWithChildren;

export function TopNavigation(props: TopNavigationProps) {
  const { children, onBack } = props;

  function _onBack() {
    onBack && onBack();
  }

  return (
    <Layout>
      <Header
        className={classNames['top-navigation-header']}
        style={{
          position: 'sticky',
          top: 0,
          zIndex: 1,
          width: '100%',
          display: 'flex',
          alignItems: 'center',
        }}
      >
        <Button
          type='primary'
          icon={<Icon type={IconClass.Back} onClick={_onBack} />}
        />
      </Header>
      <Content>{children}</Content>
    </Layout>
  );
}
