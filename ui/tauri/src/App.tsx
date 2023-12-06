import { ApplicationProvider } from './components/application';
import Router from './router';
import '/@/layout/index.less';

function App() {
  return (
    <ApplicationProvider>
      <Router />
    </ApplicationProvider>
  );
}

export default App;
