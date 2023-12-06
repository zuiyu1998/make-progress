import { defApi } from '../../utils/api/index';
import { ProjectList, ProjectForm } from './model';

export interface PageParams {
  page_size: number;
  page: number;
}

enum Api {
  CreateProject = 'create_project',
  getProjectList = 'get_project_list',
}

export async function getProjectList(params: PageParams) {
  return defApi.invoke<ProjectList>(Api.getProjectList, {
    params,
  });
}

export async function createProject(form: ProjectForm) {
  return defApi.invoke<void>(Api.CreateProject, {
    form,
  });
}
