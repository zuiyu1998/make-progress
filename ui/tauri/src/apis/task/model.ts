export type Task = {
  id: number;
  name: string;
  create_at: string;
  update_at: string;
  start_at?: string;
  project_id: number;
  plan_id: number;
  duration: number;
  real_duration: number;
  remark: string;
  status: TaskStatus;
};

export enum TaskStatus {
  Start,
  End,
  Pause,
  Playing,
}

export type TaskForm = {
  name: string;
  start_at?: number;
  duration: number;
  project_id: number;
  plan_id: number;
};

export type TaskList = {
  data: Task[];
  total: number;
  page_size: number;
  page: number;
  has_next: boolean;
};
