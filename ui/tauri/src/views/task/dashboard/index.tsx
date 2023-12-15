import { Row, Col } from 'antd';
import { getTaskList } from '/@/apis/task';
import React from 'react';
import {
  TaskItem,
  TaskItemProp,
  intoTaskItemProp,
} from '/@/views/components/task';
import { usePageConfig } from '/@/hooks/page';
import classNames from './index.module.less';
import { PageWrapper } from '/@/layout/page';

export function useContent() {
  const [data, setData] = React.useState<TaskItemProp[]>([]);

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
      const res = await getTaskList({
        page: page,
        page_size: pageSize,
      });

      setHasNext(res.has_next);

      if (res.has_next) {
        setPage(page + 1);
      }

      let newData = res.data.map((item) => intoTaskItemProp(item));

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
    <PageWrapper contentFullHeight>
      <div className={classNames['project-dashboard']}>
        <Row gutter={16}>
          {data.map((item) => {
            return (
              <Col span={12} key={item.item.id}>
                <TaskItem {...item} />
              </Col>
            );
          })}
        </Row>
      </div>
    </PageWrapper>
  );
}

export default Dashboard;
