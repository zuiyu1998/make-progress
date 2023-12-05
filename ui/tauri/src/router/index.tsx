import { LAYOUT } from './consts';
import Dashboard from '/@/views/dashboard';
import ProjectCreateView from '/@/views/project/create';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

const router = createBrowserRouter([
  {
    path: '/',
    element: <LAYOUT />,
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
            path: 'create',
            element: <ProjectCreateView />,
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
