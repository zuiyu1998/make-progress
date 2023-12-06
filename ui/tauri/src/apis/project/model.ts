export type Project = {
  id: number;
  name: String;
  background?: String;
  create_at: String;
  update_at: String;
  end_at?: String;
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
  name: String;
  end_at?: String;
};
