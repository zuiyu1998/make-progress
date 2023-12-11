import { CustomIconFontProps, Iconfont } from './src/iconfont';

export type IconProps = CustomIconFontProps;

function Icon(props: IconProps) {
  return <Iconfont {...props} />;
}

export enum IconClass {
  Time = 'icon-time',
}

export default Icon;
