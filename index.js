import init, { greet, rotate_shape, translate_shape } from "./pkg/hello_wasm.js";

init().then(() => {
  function rotate_square(length, orientation_in_degrees) {
    let svg = rotate_shape(length, orientation_in_degrees);
    let base_div = document.getElementById('base_div');
    base_div.innerHTML = svg;
  }

  function translate_square(length, x) {
    let svg = translate_shape(length, x, 0);
    let base_div = document.getElementById('base_div');
    base_div.innerHTML = svg;
  }

  function go(starting_orientation, max_duration, step, o_step) {
    function schedule(iter, duration) {
      if (duration <= 0) {
        return;
      }
      let next = iter();
      setTimeout(schedule, step, next, duration - step);
    }

    let f = (o) => {
      rotate_square(100, o);
      return () => f(o+o_step);
    };

    let iter = () => {
      return f(starting_orientation)
    }    

    let wait = 1000 - (new Date()).getMilliseconds();
    setTimeout(schedule, wait, iter, max_duration);
  }

  window.go = go;

//  document.getElementById("tagline").addEventListener('click', function() {
//   go(100, 10000, 100);
//  }, false);

});
