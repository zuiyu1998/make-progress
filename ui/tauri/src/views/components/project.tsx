import { Project } from '/@/apis/project/model';

import classNames from './project.module.less';

export type ProjectItemProp = {
  id: number;
  name: String;
  background?: String;
  create_at: String;
  update_at: String;
  end_at?: String;
};

export function intoProjectItemProp(project: Project): ProjectItemProp {
  return {
    id: project.id,
    name: project.name,
    background: project.background,
    create_at: project.create_at,
    update_at: project.update_at,
    end_at: project.end_at,
  };
}

export function ProjectItem(props: ProjectItemProp) {
  return (
    <div className={classNames.project}>
      <div>{props.background}</div>
      <div>{props.name}</div>
      <div>{props.end_at}</div>
      <div>
        <div>{props.create_at}</div>
        <div>{props.update_at}</div>
      </div>
    </div>
  );
}
