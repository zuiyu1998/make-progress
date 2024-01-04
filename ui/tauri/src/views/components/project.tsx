import { Project } from '/@/apis/project/model';
import { dateFormat, DateFormat } from '../../utils/format';
import classNames from './index.module.less';
import Icon, { IconClass } from '/@/components/icon';
import { Tooltip } from 'antd';
import { ContextMenu, MenuItem } from '/@/components/contextmenu';
import { useState } from 'react';
import { useNavigate, createSearchParams } from 'react-router-dom';

export type ProjectItemProp = {
  id: number;
  name: string;
  background?: string;
  create_at: string;
  update_at: string;
  end_at?: string;
};

export function intoProjectItemProp(project: Project): ProjectItemProp {
  return {
    id: project.id,
    name: project.name,
    background: project.background,
    create_at: project.create_at,
    update_at: project.update_at,
    end_at: project.end_at,
  };
}

export type ProjectItemBackgroudProp = {
  background?: String;
};

export function ProjectItemBackgroud(props: ProjectItemBackgroudProp) {
  const { background } = props;

  return <div className={classNames['project-background']}>{background}</div>;
}

export function ProjectItem(props: ProjectItemProp) {
  const [isOpen, setOpen] = useState(false);
  const [anchorPoint, setAnchorPoint] = useState({ x: 0, y: 0 });

  const navigate = useNavigate();

  function onCreateProject() {
    navigate({
      pathname: '/plan/create',
      search: createSearchParams({
        project_id: props.id.toString(),
      }).toString(),
    });
  }

  function goPlanList() {
    navigate({
      pathname: '/plan/dashboard',
      search: createSearchParams({
        project_id: props.id.toString(),
      }).toString(),
    });
  }

  return (
    <div
      onContextMenu={(e) => {
        if (typeof document.hasFocus === 'function' && !document.hasFocus())
          return;

        e.preventDefault();
        setAnchorPoint({ x: e.clientX, y: e.clientY });
        setOpen(true);
      }}
    >
      <div className={classNames['project']}>
        <ProjectItemBackgroud background={props.background} />
        <div className={classNames['project-bottom']}>
          <div className={classNames['project-title']}>{props.name}</div>
          <div>{props.end_at}</div>
          <div className={classNames['project-date-container']}>
            <Tooltip title={dateFormat(props.create_at)}>
              <div className={classNames['project-create-at']}>
                <div className={classNames['project-icon']}>
                  <Icon type={IconClass.Time} />
                </div>
                {dateFormat(props.create_at, DateFormat.Year)}
              </div>
            </Tooltip>
            <Tooltip title={dateFormat(props.update_at)}>
              <div className={classNames['project-update-at']}>
                <div className={classNames['project-icon']}>
                  <Icon type={IconClass.Time} />
                </div>
                {dateFormat(props.update_at, DateFormat.Year)}
              </div>
            </Tooltip>
          </div>
        </div>
      </div>

      <ContextMenu
        anchorPoint={anchorPoint}
        state={isOpen ? 'open' : 'closed'}
        direction='right'
        onClose={() => setOpen(false)}
      >
        <MenuItem onClick={onCreateProject}>创建计划</MenuItem>
        <MenuItem onClick={goPlanList}>计划列表</MenuItem>
      </ContextMenu>
    </div>
  );
}
