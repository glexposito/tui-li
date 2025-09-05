import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import Layout from './Layout';

describe('Layout', () => {
  it('should render Children component correctly', () => {
    render(
      <Layout>
        return(<h1>children</h1>)
      </Layout>
    );
    const element = screen.getByText('children');
    expect(element).toBeInTheDocument();
  });
});
