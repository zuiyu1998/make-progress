import { dateFormat, DateFormat } from '/@/utils/date_format';
import classNames from './index.module.less';
import Icon, { IconClass } from '/@/components/icon';
import { Tooltip } from 'antd';
import { ContextMenu, MenuItem } from '/@/components/contextmenu';
import { useState } from 'react';
import { useNavigate, createSearchParams } from 'react-router-dom';
import { Plan } from '/@/apis/plan/model';

export type PlanItemProp = {
  id: number;
  name: string;
  background?: string;
  create_at: string;
  update_at: string;
  dead_at: string;
  project_id: number;
};

export function intoPlanItemProp(plan: Plan): PlanItemProp {
  return {
    id: plan.id,
    name: plan.name,
    create_at: plan.create_at,
    update_at: plan.update_at,
    dead_at: plan.dead_at,
    project_id: plan.project_id,
  };
}

export type ProjectItemBackgroudProp = {
  background?: String;
};

export function ProjectItemBackgroud(props: ProjectItemBackgroudProp) {
  const { background } = props;

  return <div className={classNames['project-background']}>{background}</div>;
}

export function PlanItem(props: PlanItemProp) {
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
          <div>{props.dead_at}</div>
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
        <MenuItem onClick={onCreateProject}>创建任务</MenuItem>
      </ContextMenu>
    </div>
  );
}
