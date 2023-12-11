import { Form, Input, Button, DatePicker } from 'antd';
import { createProject } from '/@/apis/project';

function PlanCreateView() {
  const [form] = Form.useForm();

  async function onFinish() {
    try {
      const values = await form.validateFields();

      console.log(values);

      createProject({
        name: values.name,
      });
    } catch (error) {}
  }

  return (
    <div>
      <Form form={form} onFinish={onFinish}>
        <Form.Item name='name' label='名称' required>
          <Input />
        </Form.Item>
        <Form.Item name='end_at' label='预计结束时间'>
          <DatePicker />
        </Form.Item>

        <Form.Item wrapperCol={{ span: 12, offset: 6 }}>
          <Button type='primary' htmlType='submit'>
            保存
          </Button>
        </Form.Item>
      </Form>
    </div>
  );
}

export default PlanCreateView;
