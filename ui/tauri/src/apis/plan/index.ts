import { defApi } from '../../utils/api/index';
import { PlanForm, PlanList } from './model';

export interface PageParams {
  page_size: number;
  page: number;
}

enum Api {
  CreatePlan = 'create_plan',
  GetPlanList = 'get_plan_list',
}

export async function createPlan(projectId: number, form: PlanForm) {
  console.log(form, projectId);

  return defApi.invoke<void>(Api.CreatePlan, {
    form,
    projectId,
  });
}

export async function getPlanList(projectId: number, params: PageParams) {
  return defApi.invoke<PlanList>(Api.CreatePlan, {
    params,
    projectId,
  });
}
