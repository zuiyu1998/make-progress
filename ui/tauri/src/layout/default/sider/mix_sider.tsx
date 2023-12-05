import { Layout, Space, Button } from 'antd';
import { useSlider } from './use_slider';

const { Sider } = Layout;

export default function MixSider() {
  const { sliderWidth } = useSlider();

  return (
    <Sider
      trigger={null}
      collapsible
      collapsed={true}
      collapsedWidth={sliderWidth}
    ></Sider>
  );
}
