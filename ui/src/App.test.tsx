import { act, render } from '@testing-library/react';
import '@testing-library/jest-dom';
import App from './App';

describe('App', () => {
  it('should render App component correctly', async () => {
    const promise = Promise.resolve();

    render(<App />);

    await act(async () => {
      await promise;
    });
  });
});
