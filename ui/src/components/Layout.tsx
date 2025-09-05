import { useEffect } from 'react';
import Footer from './Footer';
import Header from './Header';

export default function Layout({ children }: React.PropsWithChildren) {
  useEffect(() => {
    const heightClass = 'h-100';
    document.body.classList.add(heightClass);
    const root = document.getElementById('root');

    if (root != null) {
      root.classList.add(heightClass);
    }
  });

  return (
    <>
      <div className="d-flex h-100 text-center text-bg-dark">
        <div className="cover-container d-flex w-100 h-100 p-3 mx-auto flex-column">
          <Header />
          {children}
          <Footer />
        </div>
      </div>
    </>
  );
}
