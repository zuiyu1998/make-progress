import { createFromIconfontCN } from '@ant-design/icons';
import { IconFontProps } from '@ant-design/icons/lib/components/IconFont';

const IconFont = createFromIconfontCN({
  scriptUrl: ['/iconfont/iconfont.js'],
});

export interface CustomIconFontProps extends IconFontProps {
  type: string;
}

export function Iconfont(props: CustomIconFontProps) {
  return <IconFont {...props} />;
}
