/* @refresh reload */
import { render } from 'solid-js/web';
import { Component } from 'solid-js';

const App: Component = () => {
  return (
    <div>hello world</div>
  );
};

render(() => <App />, document.getElementById('root') as HTMLElement);