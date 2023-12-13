import { Form, Input, Button, DatePicker, message, InputNumber } from 'antd';
import { createTask } from '/@/apis/task';
import { TopNavigation } from '/@/layout/page';
import { useNavigate, useSearchParams } from 'react-router-dom';
import classNames from './index.module.less';
import { timestamp } from '/@/utils/date_format';
function PlanCreateView() {
  const [form] = Form.useForm();

  const [searchParams] = useSearchParams();

  const navigate = useNavigate();

  async function onFinish() {
    try {
      const values = await form.validateFields();

      const projectId = searchParams.get('project_id');
      const planId = searchParams.get('plan_id');

      if (!projectId || !planId) {
        message.error('项目不存在');
        return;
      }

      await createTask({
        ...values,
        start_at: timestamp(values['start_at']),
        project_id: Number(projectId),
        plan_id: Number(planId),
      });
    } catch (error) {
      console.log(error);
    }
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
        <Form.Item name='start_at' label='预计开始时间'>
          <DatePicker />
        </Form.Item>
        <Form.Item name='duration' label='预计花费时间' required>
          <InputNumber />
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
