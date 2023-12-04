import Dashboard from '/@/views/dashboard';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Dashboard />,
  },
]);

function Router() {
  return <RouterProvider router={router} />;
}

export default Router;
