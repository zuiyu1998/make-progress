import { Button } from 'antd';
import { useNavigate } from 'react-router-dom';

function Dashboard() {
  const navigate = useNavigate();

  function goto() {
    navigate('/project/create');
  }

  return (
    <div className='container'>
      <Button type='primary' onClick={goto}>
        新增项目
      </Button>
    </div>
  );
}

export default Dashboard;
