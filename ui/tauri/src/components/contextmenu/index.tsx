import { ControlledMenu, ControlledMenuProps } from '@szhsin/react-menu';
import '@szhsin/react-menu/dist/index.css';

export { Menu, SubMenu, MenuItem } from '@szhsin/react-menu';

export type ContextMenuProps = ControlledMenuProps;

export function ContextMenu(props: ContextMenuProps) {
  return <ControlledMenu {...props}></ControlledMenu>;
}
