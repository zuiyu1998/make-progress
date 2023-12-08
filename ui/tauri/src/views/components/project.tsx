import { Project } from '/@/apis/project/model';
import { dateFormat } from '/@/utils/date_format';
import classNames from './index.module.less';

export type ProjectItemProp = {
  id: number;
  name: string;
  background?: string;
  create_at: string;
  update_at: string;
  end_at?: string;
};

export function intoProjectItemProp(project: Project): ProjectItemProp {
  return {
    id: project.id,
    name: project.name,
    background: project.background,
    create_at: dateFormat(project.create_at),
    update_at: dateFormat(project.update_at),
    end_at: project.end_at,
  };
}

export type ProjectItemBackgroudProp = {
  background?: String;
};

export function ProjectItemBackgroud(props: ProjectItemBackgroudProp) {
  const { background } = props;

  return <div className={classNames['project-background']}>{background}</div>;
}

export function ProjectItem(props: ProjectItemProp) {
  return (
    <div className={classNames['project']}>
      <ProjectItemBackgroud background={props.background} />
      <div className={classNames['project-bottom']}>
        <div className={classNames['project-title']}>{props.name}</div>
        <div>{props.end_at}</div>
        <div className={classNames['project-date-container']}>
          <div>{props.create_at}</div>
          <div>{props.update_at}</div>
        </div>
      </div>
    </div>
  );
}
