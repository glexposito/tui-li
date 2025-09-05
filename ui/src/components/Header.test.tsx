import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import Header from './Header';

describe('Header', () => {
  it('should render Header component correctly', () => {
    render(<Header />);
    const element = screen.getByRole('heading');
    expect(element).toBeInTheDocument();
  });
});
