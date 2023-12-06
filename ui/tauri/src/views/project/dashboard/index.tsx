import { Row, Col } from 'antd';
import { getProjectList } from '/@/apis/project';
import React from 'react';
import {
  ProjectItem,
  intoProjectItemProp,
  ProjectItemProp,
} from '/@/views/components/project';
import { usePageConfig } from '/@/hooks/page';
import classNames from './index.module.less';

export function useContent() {
  const [data, setData] = React.useState<ProjectItemProp[]>([]);

  const { page, pageSize, loading, setLoading, hasNext, setHasNext, setPage } =
    usePageConfig();

  async function getData(isClear: boolean) {
    if (loading) {
      return;
    }

    if (!hasNext) {
      return;
    }

    setLoading(true);

    try {
      const res = await getProjectList({
        page: page,
        page_size: pageSize,
      });

      setHasNext(res.has_next);

      if (res.has_next) {
        setPage(page + 1);
      }

      let newData = res.data.map((item) => intoProjectItemProp(item));

      if (isClear) {
        setData(newData);
      } else {
        setData((old) => old.concat(newData));
      }
    } catch (error) {
    } finally {
      setLoading(false);
    }
  }

  React.useEffect(() => {
    getData(true);
  }, []);

  return {
    data,
  };
}

function Dashboard() {
  const { data } = useContent();

  return (
    <div className={classNames['preject-dashboard']}>
      <Row gutter={16}>
        {data.map((item) => {
          return (
            <Col span={6} key={item.id}>
              <ProjectItem {...item} />
            </Col>
          );
        })}
      </Row>
    </div>
  );
}

export default Dashboard;
