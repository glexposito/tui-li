import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import Footer from './Footer';

describe('Footer', () => {
  it('should render Footer component correctly', () => {
    render(<Footer />);
    const element = screen.getByRole('contentinfo');
    expect(element).toBeInTheDocument();
  });
});
