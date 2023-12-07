import { defApi } from '../../utils/api/index';
import { PlanForm } from './model';

export interface PageParams {
  page_size: number;
  page: number;
}

enum Api {
  CreatePlan = 'create_plan',
}

export async function createPlan(project_id: number, form: PlanForm) {
  return defApi.invoke<void>(Api.CreatePlan, {
    form,
    project_id,
  });
}
