import { Button } from 'antd';
import { useNavigate } from 'react-router-dom';
import { getProjectList } from '/@/apis/project';
import React from 'react';

function Dashboard() {
  const navigate = useNavigate();

  function goto() {
    navigate('/project/create');
  }

  async function _getProjectList() {
    try {
      const res = await getProjectList();

      console.log(res);
    } catch (error) {}
  }

  React.useEffect(() => {
    _getProjectList();
  }, []);

  return (
    <div className='container'>
      <Button type='primary' onClick={goto}>
        新增项目
      </Button>
    </div>
  );
}

export default Dashboard;
