import classNames from './index.module.less';
import { Task } from '/@/apis/task/model';
import React from 'react';
import { Bakground } from './background';

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
      <div className={classNames['task-item-name']}>{task.name}</div>
      <div>
        <div>{task.create_at}</div>
        <div>{task.update_at}</div>
      </div>

      <div>操作</div>
    </div>
  );
}
