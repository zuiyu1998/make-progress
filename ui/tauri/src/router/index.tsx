import Dashboard from '/@/views/dashboard';
import ProjectCreateView from '/@/views/project/create';
import {
  createBrowserRouter,
  redirect,
  RouterProvider,
} from 'react-router-dom';

const router = createBrowserRouter([
  {
    path: '/',
    loader: () => {
      return redirect('/dashboard');
    },
  },
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
]);

function Router() {
  return <RouterProvider router={router} />;
}

export default Router;
