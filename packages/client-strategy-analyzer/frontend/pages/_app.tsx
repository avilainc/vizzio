import React from 'react';
import '../styles/globals.css';

interface AppProps {
  Component: React.ComponentType<any>;
  pageProps: any;
}

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <div>
      <Component {...pageProps} />
    </div>
  );
}

export default MyApp;
