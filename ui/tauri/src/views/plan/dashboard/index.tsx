import { Row, Col, message } from 'antd';
import { getPlanList } from '/@/apis/plan';
import React from 'react';
import {
  PlanItem,
  intoPlanItemProp,
  PlanItemProp,
} from '/@/views/components/plan';
import { usePageConfig } from '/@/hooks/page';
import classNames from './index.module.less';
import { useSearchParams } from 'react-router-dom';
import { PageWrapper } from '/@/layout/page';

export function useContent() {
  const [data, setData] = React.useState<PlanItemProp[]>([]);
  const [searchParams] = useSearchParams();

  const { page, pageSize, loading, setLoading, hasNext, setHasNext, setPage } =
    usePageConfig();

  async function getData(isClear: boolean) {
    const projectId = searchParams.get('project_id');

    if (!projectId) {
      message.error('项目不存在');
      return;
    }

    if (loading) {
      return;
    }

    if (!hasNext) {
      return;
    }

    setLoading(true);

    try {
      const res = await getPlanList(Number(projectId), {
        page: page,
        page_size: pageSize,
      });

      setHasNext(res.has_next);

      if (res.has_next) {
        setPage(page + 1);
      }

      let newData = res.data.map((item) => intoPlanItemProp(item));

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
              <Col span={12} key={item.id}>
                <PlanItem {...item} />
              </Col>
            );
          })}
        </Row>
      </div>
    </PageWrapper>
  );
}

export default Dashboard;
