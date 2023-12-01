import { ApplicationProvider } from './components/application';
import Router from './router';

function App() {
  return (
    <ApplicationProvider>
      <Router />
    </ApplicationProvider>
  );
}

export default App;
