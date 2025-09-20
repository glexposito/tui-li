import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Shortener from './Shortener';
import { useShortenMutation } from './useShortenMutation';

// mock the mutation hook
jest.mock('./useShortenMutation');
const mockedUseShortenMutation =
  useShortenMutation as jest.MockedFunction<typeof useShortenMutation>;

const baseMock = () =>
  ({
    mutate: jest.fn(),
    reset: jest.fn(),
    isSuccess: false,
    isError: false,
    data: undefined,
  } as any);

describe('Shortener component', () => {
  beforeEach(() => {
    jest.resetAllMocks();
  });

  it('calls reset then mutate with the typed URL', async () => {
    const user = userEvent.setup();
    const mock = baseMock();
    mockedUseShortenMutation.mockReturnValue(mock);

    render(<Shortener />);

    const input = screen.getByPlaceholderText('Enter a full URL');
    const button = screen.getByRole('button', { name: /shorten url/i });

    await user.type(input, 'https://example.com');
    await user.click(button);

    expect(mock.reset).toHaveBeenCalledTimes(1);
    expect(mock.mutate).toHaveBeenCalledWith({ long_url: 'https://example.com' });
  });

  it('renders the short link on success', () => {
    const mock = baseMock();
    mock.isSuccess = true;
    mock.data = { id: 'AbCd12', long_url: 'https://example.com' };
    mockedUseShortenMutation.mockReturnValue(mock);

    render(<Shortener />);

    const origin = window.location.origin; // jsdom default is "http://localhost"
    const expectedText = `${origin}/AbCd12`;

    expect(screen.getByText('Short link:')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: expectedText })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: expectedText })).toHaveAttribute(
      'href',
      expectedText
    );
  });

  it('renders a 429 rate-limit message with Retry-After seconds', () => {
    const mock = baseMock();
    mock.isError = true;
    mock.error = { status: 429, retryAfter: 7 };
    mockedUseShortenMutation.mockReturnValue(mock);

    render(<Shortener />);

    expect(screen.getByText(/hit the rate limit/i)).toBeInTheDocument();
    expect(screen.getByText(/try again in about 7 seconds/i)).toBeInTheDocument();
  });

  it('renders a 500 server error message', () => {
    const mock = baseMock();
    mock.isError = true;
    mock.error = { status: 500 };
    mockedUseShortenMutation.mockReturnValue(mock);

    render(<Shortener />);

    expect(
      screen.getByText(/oops, something went wrong on our servers/i)
    ).toBeInTheDocument();
    expect(screen.getByText(/please try again later/i)).toBeInTheDocument();
  });

  it('renders a generic error message for unhandled status', () => {
    const mock = baseMock();
    mock.isError = true;
    mock.error = { status: 418 }; // I’m a teapot 😅
    mockedUseShortenMutation.mockReturnValue(mock);

    render(<Shortener />);

    expect(
      screen.getByText(/there was an error shortening your url/i)
    ).toBeInTheDocument();
  });

  it('copies the short link to clipboard when clicking the copy button', async () => {
    const user = userEvent.setup();

    // mock clipboard
    const writeText = jest.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, 'clipboard', {
      value: { writeText },
      configurable: true,
    });

    const mock = baseMock();
    mock.isSuccess = true;
    mock.data = { id: 'K4PkNm', long_url: 'https://google.com' };
    mockedUseShortenMutation.mockReturnValue(mock);

    render(<Shortener />);

    const copyBtn = screen.getByRole('button', { name: /copy short link/i });
    await user.click(copyBtn);

    const origin = window.location.origin;
    expect(writeText).toHaveBeenCalledWith(`${origin}/K4PkNm`);
  });
});
