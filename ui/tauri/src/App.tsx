import { ApplicationProvider } from './components/application';
import Router from './router';
import { AppLayout } from '/@/layout';
import '/@/layout/index.less';

function App() {
  return (
    <ApplicationProvider>
      <AppLayout>
        <Router />
      </AppLayout>
    </ApplicationProvider>
  );
}

export default App;
