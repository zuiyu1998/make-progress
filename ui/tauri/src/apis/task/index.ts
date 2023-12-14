import { defApi } from '../../utils/api/index';
import { TaskForm, TaskList } from './model';

export interface PageParams {
  page_size: number;
  page: number;
  project_id: number;
  plan_id: number;
}

enum Api {
  CreateTask = 'create_task',
  GetTaskList = 'get_task_list',
}

export async function createTask(form: TaskForm) {
  return defApi.invoke<void>(Api.CreateTask, {
    form,
  });
}

export async function getTaskList(params: TaskForm) {
  return defApi.invoke<TaskList>(Api.GetTaskList, {
    params,
  });
}
