import { Form, Input, Button, DatePicker, message } from 'antd';
import { createPlan } from '/@/apis/plan';
import { TopNavigation } from '/@/layout/page';
import { useNavigate, useSearchParams } from 'react-router-dom';
import classNames from './index.module.less';
function PlanCreateView() {
  const [form] = Form.useForm();

  const [searchParams] = useSearchParams();

  const navigate = useNavigate();

  async function onFinish() {
    try {
      const values = await form.validateFields();

      const projectId = searchParams.get('project_id');

      if (!projectId) {
        message.error('项目不存在');
        return;
      }

      await createPlan(Number(projectId), { ...values });
    } catch (error) {}
  }

  return (
    <TopNavigation
      onBack={() => {
        navigate(-1);
      }}
    >
      <Form
        className={classNames['plan-create-form']}
        form={form}
        onFinish={onFinish}
        labelCol={{
          style: {
            width: '150px',
          },
        }}
      >
        <Form.Item name='name' label='名称' required>
          <Input />
        </Form.Item>
        <Form.Item name='dead_at' label='预计结束时间' required>
          <DatePicker />
        </Form.Item>

        <Form.Item
          wrapperCol={{
            style: {
              marginLeft: '150px',
            },
          }}
        >
          <Button type='primary' htmlType='submit'>
            保存
          </Button>
        </Form.Item>
      </Form>
    </TopNavigation>
  );
}

export default PlanCreateView;
