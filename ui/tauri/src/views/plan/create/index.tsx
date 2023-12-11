import { Form, Input, Button, DatePicker } from 'antd';
import { createProject } from '/@/apis/project';
import { TopNavigation } from '/@/layout/page';
import { useNavigate } from 'react-router-dom';

function PlanCreateView() {
  const [form] = Form.useForm();

  const navigate = useNavigate();

  async function onFinish() {
    try {
      const values = await form.validateFields();

      createProject({
        name: values.name,
      });
    } catch (error) {}
  }

  return (
    <TopNavigation
      onBack={() => {
        navigate(-1);
      }}
    >
      <Form form={form} onFinish={onFinish}>
        <Form.Item name='name' label='名称' required>
          <Input />
        </Form.Item>
        <Form.Item name='end_at' label='预计结束时间'>
          <DatePicker />
        </Form.Item>

        <Form.Item>
          <Button type='primary' htmlType='submit'>
            保存
          </Button>
        </Form.Item>
      </Form>
    </TopNavigation>
  );
}

export default PlanCreateView;
