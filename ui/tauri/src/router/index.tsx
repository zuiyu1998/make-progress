import Dashboard from '/@/views/dashboard';
import ProjectCreate from '/@/views/project/create';
import ProjectDashboard from '/@/views/project/dashboard';
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
    ],
  },
]);

function Router() {
  return <RouterProvider router={router} />;
}

export default Router;
