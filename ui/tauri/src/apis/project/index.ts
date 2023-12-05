import { defApi } from '../../utils/api/index';
import { ProjectList, ProjectForm } from './model';

enum Api {
  CreateProject = 'create_project',
  getProjectList = 'get_project_list',
}

export async function getProjectList() {
  return defApi.invoke<ProjectList>(Api.getProjectList, {
    params: {
      page_size: 50,
      page: 0,
    },
  });
}

export async function createProject(form: ProjectForm) {
  return defApi.invoke<void>(Api.CreateProject, {
    form,
  });
}
