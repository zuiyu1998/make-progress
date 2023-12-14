import { dateFormat, DateFormat } from '/@/utils/date_format';
import classNames from './index.module.less';
import Icon, { IconClass } from '/@/components/icon';
import { Tooltip } from 'antd';
import { Task, TaskStatus } from '/@/apis/task/model';

export type TaskItemProp = {
  id: number;
  name: string;
  background?: string;
  create_at: string;
  update_at: string;
  start_at?: string;
  project_id: number;
  plan_id: number;
  duration: number;
  real_duration: number;
  remark: string;
  status: TaskStatus;
};

export function intoPlanItemProp(task: Task): TaskItemProp {
  return {
    id: task.id,
    name: task.name,
    create_at: task.create_at,
    update_at: task.update_at,
    project_id: task.project_id,
    plan_id: task.plan_id,
    duration: task.duration,
    real_duration: task.real_duration,
    remark: task.remark,
    status: task.status,
  };
}

export type ProjectItemBackgroudProp = {
  background?: String;
};

export function TaskItemBackgroud(props: ProjectItemBackgroudProp) {
  const { background } = props;

  return <div className={classNames['project-background']}>{background}</div>;
}

export function TaskItem(props: TaskItemProp) {
  return (
    <div className={classNames['project']}>
      <TaskItemBackgroud background={props.background} />
      <div className={classNames['project-bottom']}>
        <div className={classNames['project-title']}>{props.name}</div>
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
  );
}
