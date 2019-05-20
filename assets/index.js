import { importWasm } from './loadWasm.js';
import { display_one_sl, num_cols } from './pkg/sl-rs.js';

function init() {
  animateSL();
}

function animateSL() {
  var x = num_cols() - 1;
  setInterval(() => {
    document.getElementById('test-terminal').innerHTML = display_one_sl(x--);
    if (x < -num_cols()) {
      x = num_cols() - 1;
    }
  }, 40);
}

window.onload = () => importWasm().then(init);
