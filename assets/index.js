import { importWasm } from './loadWasm.js';
import { display_one_sl } from './pkg/sl-rs.js';

function init() {
  console.log('Hello from JavaScript!');
  document.getElementById('test-terminal').innerHTML = 'Hello from JS';
  console.log(display_one_sl());
}

window.onload = () => importWasm().then(init);
