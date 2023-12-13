import { defApi } from '../../utils/api/index';
import { TaskForm } from './model';

export interface PageParams {
  page_size: number;
  page: number;
}

enum Api {
  CreateTask = 'create_task',
  GetPlanList = 'get_plan_list',
}

export async function createTask(
  projectId: number,
  planId: number,
  form: TaskForm
) {
  return defApi.invoke<void>(Api.CreateTask, {
    form,
    projectId,
    planId,
  });
}
