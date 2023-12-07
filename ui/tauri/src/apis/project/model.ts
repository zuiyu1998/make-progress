export type Project = {
  id: number;
  name: string;
  background?: string;
  create_at: string;
  update_at: string;
  end_at?: string;
  link_list: Link[];
};

export type Link = {};

export type ProjectList = {
  data: Project[];
  total: number;
  page_size: number;
  page: number;
  has_next: boolean;
};

export type ProjectForm = {
  name: string;
  end_at?: string;
};
