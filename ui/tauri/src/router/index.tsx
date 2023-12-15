import Dashboard from '/@/views/dashboard';
import ProjectCreate from '/@/views/project/create';
import PlanCreate from '/@/views/plan/create';
import TaskCreate from '/@/views/task/create';
import ProjectDashboard from '/@/views/project/dashboard';
import PlanDashboard from '/@/views/plan/dashboard';
import TaskDashboard from '/@/views/task/dashboard';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

const router = createBrowserRouter([
  {
    path: '/',
    children: [
      { index: true, element: <Dashboard /> },
      {
        path: '/dashboard',
        element: <Dashboard />,
      },
      {
        path: '/project',
        children: [
          {
            path: 'dashboard',
            element: <ProjectDashboard />,
          },
          {
            path: 'create',
            element: <ProjectCreate />,
          },
        ],
      },
      {
        path: '/plan',
        children: [
          {
            path: 'dashboard',
            element: <PlanDashboard />,
          },
          {
            path: 'create',
            element: <PlanCreate />,
          },
        ],
      },
      {
        path: '/task',
        children: [
          {
            path: 'dashboard',
            element: <TaskDashboard />,
          },
          {
            path: 'create',
            element: <TaskCreate />,
          },
        ],
      },
    ],
  },
]);

function Router() {
  return <RouterProvider router={router} />;
}

export default Router;
