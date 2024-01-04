import classNames from './index.module.less';
import { Task } from '/@/apis/task/model';
import React from 'react';
import { Bakground } from './background';
import Icon, { IconClass } from '/@/components/icon';
import { dateFormat } from '/@/utils/format';

export type TaskItemProp = {
  item: Task;
};

export function intoTaskItemProp(task: Task): TaskItemProp {
  return {
    item: task,
  };
}

export function useTaskHook(task: Task) {
  const start = React.useCallback(() => {
    function _start() {}

    return _start;
  }, [task]);

  return {
    start,
    task,
  };
}

export function TaskItem(props: TaskItemProp) {
  const { task } = useTaskHook(props.item);

  return (
    <div className={classNames['task-item']}>
      <Bakground background={task.background} alt={task.name} />

      <div className={classNames['task-item-content']}>
        <div className={classNames['task-item-name']}>{task.name}</div>

        <div className={classNames['task-item-row']}>
          <div className={classNames['task-item-icon']}>
            <Icon type={IconClass.Time} size={18} />
            <div className={classNames['task-item-icon-text']}>
              {dateFormat(task.create_at)}
            </div>
          </div>
          <div className={classNames['task-item-icon']}>
            <Icon type={IconClass.Time} size={18} />
            <div className={classNames['task-item-icon-text']}>
              {dateFormat(task.create_at)}
            </div>
          </div>
        </div>

        <div className={classNames['task-item-actions']}></div>
      </div>
    </div>
  );
}
