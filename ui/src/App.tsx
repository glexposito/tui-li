import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Shortener from './components/Shortener';

const queryClient = new QueryClient();

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Shortener />
    </QueryClientProvider>
  );
}
