import init, { greet, render_rotated, render_translated } from "./pkg/hello_wasm.js";
import { RenderedShape } from './pkg/hello_wasm.js';

init().then(() => {
  function rotate_square(length, orientation_in_degrees) {
    let rendered_shape = render_rotated(length, orientation_in_degrees);
    document.getElementById('base_div').innerHTML = rendered_shape.svg;
  }

  function translate_square(length, x) {
    let rendered_shape = render_translated(length, x);
    document.getElementById('base_div').innerHTML = rendered_shape.svg;
  }

  function go(starting_orientation, max_duration, step, o_step) {
    function schedule(iter, duration) {
      if (duration <= 0) {
        return;
      }
      setTimeout(schedule, step, iter(), duration - step);
    }

    let f = (o) => {
      console.log(`iterating: f(${o})`);
      rotate_square(100, o);
      return () => f(o+o_step);
    };

    let iter = () => {
      return f(starting_orientation)
    }

    schedule(iter, max_duration);
  }

  window.go = go;
  window.showme = () => go(0, 5000, 50, 2);

//  document.getElementById("tagline").addEventListener('click', function() {
//   go(100, 10000, 100);
//  }, false);

});
