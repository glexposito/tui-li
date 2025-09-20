import { useQuery } from '@tanstack/react-query';

export const useRandomQuoteQuery = () => {
  return useQuery({
    queryKey: ['RandomQuote'],
    queryFn: async () => {
      const res = await fetch('https://dummyjson.com/quotes/random');
      if (!res.ok) {
        console.log(res.statusText);
        throw new Error(res.statusText);
      }
      return res.json();
    },
  });
};
