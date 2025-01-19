import { Navigate, RouteObject } from 'react-router-dom';
import SubLayout from '../layout/subpage';
import DevelopePage from '../pages/DevelopePage';

const routes: RouteObject[] = [{
    path: '/',
    element: <HomePage/>
  },{
    Component: SubLayout,
    children: [
      {
        path: '/develop',
        element: <DevelopePage />
      }
    ]
  },
  // ...external,
  {
    path: '/*',
    element: <Navigate to="/" />
  }
];
export default routes;