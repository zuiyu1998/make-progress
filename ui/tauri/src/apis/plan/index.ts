import { defApi } from '../../utils/api/index';
import { PlanForm } from './model';

export interface PageParams {
  page_size: number;
  page: number;
}

enum Api {
  CreatePlan = 'create_plan',
  GetPlanList = 'get_plan_list',
}

export async function createPlan(project_id: number, form: PlanForm) {
  return defApi.invoke<void>(Api.CreatePlan, {
    form,
    project_id,
  });
}

export async function getPlanList(project_id: number, params: PageParams) {
  return defApi.invoke<void>(Api.CreatePlan, {
    params,
    project_id,
  });
}
