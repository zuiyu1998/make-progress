export type Plan = {
  id: number;
  name: string;
  create_at: string;
  update_at: string;
  dead_at: string;
  project_id: number;
};

export type PlanForm = {
  name: string;
  dead_at: string;
};
